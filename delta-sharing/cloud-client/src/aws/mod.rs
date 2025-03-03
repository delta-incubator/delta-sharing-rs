use std::sync::Arc;

use self::checksum::Checksum;
use crate::aws::builder::S3EncryptionHeaders;
use crate::retry::RetryExt;
use crate::{ClientOptions, Result, RetryConfig};
use hyper::http::HeaderName;
use hyper::{http, HeaderMap};
use reqwest::{Client as ReqwestClient, IntoUrl, Method, RequestBuilder, Response};
use ring::digest;
use serde::Serialize;

use self::credential::{AwsAuthorizer, AwsCredential, CredentialExt};
use crate::util::STRICT_ENCODE_SET;
use crate::CredentialProvider;

mod builder;
mod checksum;
mod credential;

pub use builder::*;

/// This struct is used to maintain the URI path encoding
const STRICT_PATH_ENCODE_SET: percent_encoding::AsciiSet = STRICT_ENCODE_SET.remove(b'/');

/// [`CredentialProvider`] for [`AmazonS3`]
pub type AwsCredentialProvider = Arc<dyn CredentialProvider<Credential = AwsCredential>>;

const VERSION_HEADER: &str = "x-amz-version-id";
const SHA256_CHECKSUM: &str = "x-amz-checksum-sha256";
const USER_DEFINED_METADATA_HEADER_PREFIX: &str = "x-amz-meta-";
const ALGORITHM: &str = "x-amz-checksum-algorithm";

/// A specialized `Error` for object store-related errors
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Error performing DeleteObjects request: {}", source)]
    DeleteObjectsRequest { source: crate::retry::Error },

    #[error(
        "DeleteObjects request failed for key {}: {} (code: {})",
        path,
        message,
        code
    )]
    DeleteFailed {
        path: String,
        code: String,
        message: String,
    },

    #[error("Error getting DeleteObjects response body: {}", source)]
    DeleteObjectsResponse { source: reqwest::Error },

    #[error("Got invalid DeleteObjects response: {}", source)]
    InvalidDeleteObjectsResponse {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        Self::Generic {
            source: Box::new(err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AmazonConfig {
    pub region: String,
    pub credentials: AwsCredentialProvider,
    pub session_provider: Option<AwsCredentialProvider>,
    pub retry_config: RetryConfig,
    pub client_options: ClientOptions,
    pub sign_payload: bool,
    pub skip_signature: bool,
    pub disable_tagging: bool,
    pub checksum: Option<Checksum>,
    pub request_payer: bool,
    pub(crate) encryption_headers: S3EncryptionHeaders,
}

impl AmazonConfig {
    async fn get_session_credential(&self) -> Result<SessionCredential<'_>> {
        let credential = match self.skip_signature {
            false => {
                let provider = self.session_provider.as_ref().unwrap_or(&self.credentials);
                Some(provider.get_credential().await?)
            }
            true => None,
        };

        Ok(SessionCredential {
            credential,
            session_token: self.session_provider.is_some(),
            config: self,
        })
    }

    pub(crate) async fn get_credential(&self) -> Result<Option<Arc<AwsCredential>>> {
        Ok(match self.skip_signature {
            false => Some(self.credentials.get_credential().await?),
            true => None,
        })
    }

    #[inline]
    pub(crate) fn is_s3_express(&self) -> bool {
        self.session_provider.is_some()
    }
}

struct SessionCredential<'a> {
    credential: Option<Arc<AwsCredential>>,
    session_token: bool,
    config: &'a AmazonConfig,
}

impl SessionCredential<'_> {
    fn authorizer(&self) -> Option<AwsAuthorizer<'_>> {
        let mut authorizer =
            AwsAuthorizer::new(self.credential.as_deref()?, "s3", &self.config.region)
                .with_sign_payload(self.config.sign_payload)
                .with_request_payer(self.config.request_payer);

        if self.session_token {
            let token = HeaderName::from_static("x-amz-s3session-token");
            authorizer = authorizer.with_token_header(token)
        }

        Some(authorizer)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error(transparent)]
    Generic {
        #[from]
        source: crate::Error,
    },

    #[error("Retry")]
    Retry { source: crate::retry::Error },
}

impl From<RequestError> for crate::Error {
    fn from(value: RequestError) -> Self {
        match value {
            RequestError::Generic { source } => source,
            RequestError::Retry { source } => source.error(),
        }
    }
}

/// A builder for a request allowing customisation of the headers and query string
pub(crate) struct Request<'a> {
    config: &'a AmazonConfig,
    builder: RequestBuilder,
    payload_sha256: Option<digest::Digest>,
    use_session_creds: bool,
    idempotent: bool,
    retry_on_conflict: bool,
}

impl Request<'_> {
    pub(crate) fn query<T: Serialize + ?Sized + Sync>(self, query: &T) -> Self {
        let builder = self.builder.query(query);
        Self { builder, ..self }
    }

    pub(crate) fn header<K>(self, k: K, v: &str) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
    {
        let builder = self.builder.header(k, v);
        Self { builder, ..self }
    }

    pub(crate) fn headers(self, headers: HeaderMap) -> Self {
        let builder = self.builder.headers(headers);
        Self { builder, ..self }
    }

    pub(crate) fn idempotent(self, idempotent: bool) -> Self {
        Self { idempotent, ..self }
    }

    pub(crate) fn retry_on_conflict(self, retry_on_conflict: bool) -> Self {
        Self {
            retry_on_conflict,
            ..self
        }
    }

    pub(crate) fn with_encryption_headers(self) -> Self {
        let headers = self.config.encryption_headers.clone().into();
        let builder = self.builder.headers(headers);
        Self { builder, ..self }
    }

    pub(crate) fn with_session_creds(self, use_session_creds: bool) -> Self {
        Self {
            use_session_creds,
            ..self
        }
    }

    pub(crate) async fn send(self) -> Result<Response, RequestError> {
        let credential = match self.use_session_creds {
            true => self.config.get_session_credential().await?,
            false => SessionCredential {
                credential: self.config.get_credential().await?,
                session_token: false,
                config: self.config,
            },
        };

        let sha = self.payload_sha256.as_ref().map(|x| x.as_ref());

        self.builder
            .with_aws_sigv4(credential.authorizer(), sha)
            .retryable(&self.config.retry_config)
            .retry_on_conflict(self.retry_on_conflict)
            .idempotent(self.idempotent)
            .send()
            .await
            .map_err(|source| RequestError::Retry { source })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AmazonClient {
    pub config: AmazonConfig,
    pub client: ReqwestClient,
}

impl AmazonClient {
    pub(crate) fn try_new(config: AmazonConfig) -> Result<Self> {
        let client = config.client_options.client()?;
        Ok(Self { config, client })
    }

    pub(crate) fn request<U: IntoUrl>(&self, method: Method, url: U) -> Request<'_> {
        Request {
            builder: self.client.request(method, url),
            payload_sha256: None,
            config: &self.config,
            use_session_creds: true,
            idempotent: false,
            retry_on_conflict: false,
        }
    }
}
