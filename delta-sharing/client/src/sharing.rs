use delta_sharing_core::types as t;
use reqwest::{header, Client, Method};

use crate::client::retry::RetryExt;
use crate::{ClientOptions, CredentialProvider, Error, Result, RetryConfig};

pub struct DeltaSharingClient {
    client: Client,
    endpoint: url::Url,
    credential_provider: Box<dyn CredentialProvider<Credential = String>>,
    retry_config: RetryConfig,
}

impl DeltaSharingClient {
    pub fn try_new(
        endpoint: url::Url,
        credential_provider: Box<dyn CredentialProvider<Credential = String>>,
        options: Option<ClientOptions>,
        retry_config: Option<RetryConfig>,
    ) -> Result<Self> {
        Ok(Self {
            client: options.unwrap_or_default().client()?,
            endpoint,
            credential_provider,
            retry_config: retry_config.unwrap_or_default(),
        })
    }

    pub async fn list_shares(
        &self,
        request: t::ListSharesRequest,
    ) -> Result<t::ListSharesResponse> {
        let url = self.endpoint.join("shares")?;
        let cred = self.credential_provider.get_credential().await?;

        let mut req = self
            .client
            .request(Method::GET, url)
            .header(header::AUTHORIZATION, cred.as_str());

        if let Some(max_results) = request.max_results {
            req = req.query(&[("maxResults", max_results)]);
        }
        if let Some(page_token) = request.page_token {
            req = req.query(&[("pageToken", page_token)]);
        }

        let body = req
            .send_retry(&self.retry_config)
            .await
            .map_err(|e| Error::Generic {
                source: Box::new(e),
            })?
            .bytes()
            .await
            .map_err(|e| Error::Generic {
                source: Box::new(e),
            })?;

        serde_json::from_slice(&body).map_err(|e| Error::Generic {
            source: Box::new(e),
        })
    }
}
