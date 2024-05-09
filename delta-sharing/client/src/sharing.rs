use delta_sharing_core::{types as t, ListSharesRequest, Share};
use futures::{Stream, TryStreamExt};
use reqwest::{header, Client, Method};

use crate::client::{pagination::stream_paginated, retry::RetryExt};
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

    async fn list_shares_inner(
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

    pub async fn list_shares(
        &self,
        max_results: Option<i32>,
    ) -> impl Stream<Item = Result<Share>> + '_ {
        stream_paginated(max_results, move |max_results, page_token| async move {
            let request = ListSharesRequest {
                max_results,
                page_token,
            };
            self.list_shares_inner(request).await.map(|mut resp| {
                let max_results = max_results
                    .map(|m| m - resp.items.len() as i32)
                    .filter(|m| *m > 0);
                // NOTE checking if the next page token should be set to None should not be necessary
                // as the server should return None if there are no more pages to fetch to get max_results.
                let page_token = resp.next_page_token.take();
                (resp, max_results, page_token)
            })
        })
        .map_ok(|resp| futures::stream::iter(resp.items.into_iter().map(Ok)))
        .try_flatten()
    }
}
