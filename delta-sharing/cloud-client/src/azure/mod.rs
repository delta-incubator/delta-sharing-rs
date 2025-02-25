use reqwest::Client as ReqwestClient;
use std::sync::Arc;
use url::Url;

use self::credentials::AzureCredential;
use crate::{ClientOptions, CredentialProvider, Result, RetryConfig};

mod credentials;

pub type AzureCredentialProvider = Arc<dyn CredentialProvider<Credential = AzureCredential>>;

/// Configuration for [AzureClient]
#[derive(Debug)]
pub(crate) struct AzureConfig {
    pub account: String,
    pub container: String,
    pub credentials: AzureCredentialProvider,
    pub retry_config: RetryConfig,
    pub service: Url,
    pub is_emulator: bool,
    pub skip_signature: bool,
    pub disable_tagging: bool,
    pub client_options: ClientOptions,
}

impl AzureConfig {
    async fn get_credential(&self) -> Result<Option<Arc<AzureCredential>>> {
        if self.skip_signature {
            Ok(None)
        } else {
            Some(self.credentials.get_credential().await).transpose()
        }
    }
}

#[derive(Debug)]
pub(crate) struct AzureClient {
    config: AzureConfig,
    client: ReqwestClient,
}

impl AzureClient {
    /// create a new instance of [AzureClient]
    pub(crate) fn new(config: AzureConfig) -> Result<Self> {
        let client = config.client_options.client()?;
        Ok(Self { config, client })
    }

    /// Returns the config
    pub(crate) fn config(&self) -> &AzureConfig {
        &self.config
    }

    async fn get_credential(&self) -> Result<Option<Arc<AzureCredential>>> {
        self.config.get_credential().await
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
