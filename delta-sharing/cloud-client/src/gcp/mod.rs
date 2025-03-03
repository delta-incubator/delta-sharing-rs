use std::sync::Arc;

use reqwest::header::HeaderName;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use self::credential::GcpCredential;
use crate::CredentialProvider;
use crate::{ClientOptions, Result, RetryConfig};

pub use builder::*;

mod builder;
mod credential;

/// [`CredentialProvider`] for [`GoogleCloudStorage`]
pub type GcpCredentialProvider = Arc<dyn CredentialProvider<Credential = GcpCredential>>;

const VERSION_HEADER: &str = "x-goog-generation";
const DEFAULT_CONTENT_TYPE: &str = "application/octet-stream";
const USER_DEFINED_METADATA_HEADER_PREFIX: &str = "x-goog-meta-";

static VERSION_MATCH: HeaderName = HeaderName::from_static("x-goog-if-generation-match");

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Error getting put response body: {}", source)]
    PutResponseBody { source: reqwest::Error },

    #[error("Got invalid put request: {}", source)]
    InvalidPutRequest { source: quick_xml::se::SeError },

    #[error("Got invalid put response: {}", source)]
    InvalidPutResponse { source: quick_xml::de::DeError },

    #[error("Version required for conditional update")]
    MissingVersion,

    #[error("Got invalid multipart response: {}", source)]
    InvalidMultipartResponse { source: quick_xml::de::DeError },

    #[error("Got invalid signing blob response: {}", source)]
    InvalidSignBlobResponse { source: reqwest::Error },

    #[error("Got invalid signing blob signature: {}", source)]
    InvalidSignBlobSignature { source: base64::DecodeError },
}

impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        Self::Generic {
            source: Box::new(err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GoogleConfig {
    pub credentials: GcpCredentialProvider,

    pub retry_config: RetryConfig,

    pub client_options: ClientOptions,
}

impl GoogleConfig {
    pub(crate) fn new(
        credentials: GcpCredentialProvider,
        retry_config: RetryConfig,
        client_options: ClientOptions,
    ) -> Self {
        Self {
            credentials,
            retry_config,
            client_options,
        }
    }

    pub(crate) async fn get_credential(&self) -> Result<Arc<GcpCredential>> {
        self.credentials.get_credential().await
    }
}

/// Sign Blob Request Body
#[derive(Debug, Serialize)]
struct SignBlobBody {
    /// The payload to sign
    payload: String,
}

/// Sign Blob Response
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignBlobResponse {
    /// The signature for the payload
    signed_blob: String,
}

#[derive(Debug, Clone)]
pub(crate) struct GoogleClient {
    config: GoogleConfig,
    client: Client,
}

impl GoogleClient {
    pub(crate) fn try_new(config: GoogleConfig) -> Result<Self> {
        let client = config.client_options.client()?;
        Ok(Self { config, client })
    }

    pub(crate) fn config(&self) -> &GoogleConfig {
        &self.config
    }

    async fn get_credential(&self) -> Result<Arc<GcpCredential>> {
        self.config.credentials.get_credential().await
    }
}
