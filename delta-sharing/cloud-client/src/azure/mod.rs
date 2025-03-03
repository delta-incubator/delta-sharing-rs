use credential::AzureCredentialExt;
use reqwest::{Client as ReqwestClient, IntoUrl, Method, RequestBuilder};
use std::sync::Arc;
use url::Url;

use self::credential::AzureCredential;
use crate::retry::RetryExt;
use crate::{ClientOptions, CredentialProvider, Result, RetryConfig};

mod builder;
pub(crate) mod credential;

pub(crate) use self::credential::*;
pub use builder::*;

pub type AzureCredentialProvider = Arc<dyn CredentialProvider<Credential = AzureCredential>>;

/// Configuration for [AzureClient]
#[derive(Debug, Clone)]
pub struct AzureConfig {
    pub credentials: AzureCredentialProvider,
    pub retry_config: RetryConfig,
    pub skip_signature: bool,
    pub disable_tagging: bool,
    pub client_options: ClientOptions,
}

impl AzureConfig {
    pub(crate) async fn get_credential(&self) -> Result<Option<Arc<AzureCredential>>> {
        if self.skip_signature {
            Ok(None)
        } else {
            Some(self.credentials.get_credential().await).transpose()
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AzureClient {
    config: AzureConfig,
    client: ReqwestClient,
}

impl AzureClient {
    /// create a new instance of [AzureClient]
    pub(crate) fn try_new(config: AzureConfig) -> Result<Self> {
        let client = config.client_options.client()?;
        Ok(Self { config, client })
    }

    /// Returns the config
    pub(crate) fn config(&self) -> &AzureConfig {
        &self.config
    }

    pub(crate) async fn get_credential(&self) -> Result<Option<Arc<AzureCredential>>> {
        self.config.get_credential().await
    }

    pub async fn request<U>(&self, method: Method, url: U) -> Result<RequestBuilder>
    where
        U: IntoUrl,
    {
        let credential = self.get_credential().await?;
        let sensitive = credential
            .as_deref()
            .map(|c| c.sensitive_request())
            .unwrap_or_default();
        Ok(self
            .client
            .request(method, url)
            .with_azure_authorization(&credential))
    }

    // TODO: move this into higher level client ...
    // Make a Get User Delegation Key request
    // <https://docs.microsoft.com/en-us/rest/api/storageservices/get-user-delegation-key>
    // async fn get_user_delegation_key(
    //     &self,
    //     start: &DateTime<Utc>,
    //     end: &DateTime<Utc>,
    // ) -> Result<UserDelegationKey> {
    //     let credential = self.get_credential().await?;
    //     let url = self.config.service.clone();

    //     let start = start.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    //     let expiry = end.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    //     let mut body = String::new();
    //     body.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<KeyInfo>\n");
    //     body.push_str(&format!(
    //         "\t<Start>{start}</Start>\n\t<Expiry>{expiry}</Expiry>\n"
    //     ));
    //     body.push_str("</KeyInfo>");

    //     let sensitive = credential
    //         .as_deref()
    //         .map(|c| c.sensitive_request())
    //         .unwrap_or_default();
    //     let response = self
    //         .client
    //         .request(Method::POST, url)
    //         .body(body)
    //         .query(&[("restype", "service"), ("comp", "userdelegationkey")])
    //         .with_azure_authorization(&credential, &self.config.account)
    //         .retryable(&self.config.retry_config)
    //         .sensitive(sensitive)
    //         .idempotent(true)
    //         .send()
    //         .await
    //         .map_err(|source| Error::DelegationKeyRequest { source })?
    //         .bytes()
    //         .await
    //         .map_err(|source| Error::DelegationKeyResponseBody { source })?;

    //     let response: UserDelegationKey = quick_xml::de::from_reader(response.reader())
    //         .map_err(|source| Error::DelegationKeyResponse { source })?;

    //     Ok(response)
    // }
}
