use cloud_client::CloudClient;
use futures::stream::BoxStream;
use futures::{Future, Stream, StreamExt, TryStreamExt};

use crate::api::catalogs::CatalogClient;
use crate::api::credentials::CredentialsClient;
use crate::api::external_locations::ExternalLocationsClient;
use crate::api::recipients::RecipientsClient;
use crate::api::schemas::SchemasClient;
use crate::api::shares::SharesClient;
use crate::models::catalogs::v1 as catalog;
use crate::{Error, Result};

pub struct UnityCatalogClient {
    client: CloudClient,
    base_url: url::Url,
}

impl UnityCatalogClient {
    pub fn new(client: CloudClient, base_url: url::Url) -> Self {
        Self { client, base_url }
    }

    pub fn catalogs(&self) -> CatalogClient {
        CatalogClient::new(self.client.clone(), self.base_url.clone())
    }

    pub fn credentials(&self) -> CredentialsClient {
        CredentialsClient::new(self.client.clone(), self.base_url.clone())
    }

    pub fn external_locations(&self) -> ExternalLocationsClient {
        ExternalLocationsClient::new(self.client.clone(), self.base_url.clone())
    }

    pub fn recipients(&self) -> RecipientsClient {
        RecipientsClient::new(self.client.clone(), self.base_url.clone())
    }

    pub fn schemas(&self) -> SchemasClient {
        SchemasClient::new(self.client.clone(), self.base_url.clone())
    }

    pub fn shares(&self) -> SharesClient {
        SharesClient::new(self.client.clone(), self.base_url.clone())
    }
}

impl CatalogClient {
    pub fn list(
        &self,
        max_results: impl Into<Option<i32>>,
    ) -> BoxStream<'_, Result<catalog::CatalogInfo>> {
        let max_results = max_results.into();
        stream_paginated(max_results, move |max_results, page_token| async move {
            let request = catalog::ListCatalogsRequest {
                max_results,
                page_token,
            };
            let res = self
                .list_catalogs(&request)
                .await
                .map_err(|e| Error::generic(e.to_string()))?;
            Ok((res.catalogs, max_results, res.next_page_token))
        })
        .map_ok(|resp| futures::stream::iter(resp.into_iter().map(Ok)))
        .try_flatten()
        .boxed()
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        comment: impl Into<Option<String>>,
    ) -> Result<catalog::CatalogInfo> {
        let request = catalog::CreateCatalogRequest {
            name: name.into(),
            comment: comment.into(),
            ..Default::default()
        };
        self.create_catalog(&request).await
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<catalog::CatalogInfo> {
        let request = catalog::GetCatalogRequest {
            name: name.into(),
            include_browse: None,
        };
        self.get_catalog(&request).await
    }

    pub async fn delete(
        &self,
        name: impl Into<String>,
        force: impl Into<Option<bool>>,
    ) -> Result<()> {
        let request = catalog::DeleteCatalogRequest {
            name: name.into(),
            force: force.into(),
        };
        self.delete_catalog(&request).await
    }
}

pub fn stream_paginated<F, Fut, S, T>(state: S, op: F) -> impl Stream<Item = Result<T>>
where
    F: Fn(S, Option<String>) -> Fut + Copy,
    Fut: Future<Output = Result<(T, S, Option<String>)>>,
{
    enum PaginationState<T> {
        Start(T),
        HasMore(T, String),
        Done,
    }

    futures::stream::unfold(PaginationState::Start(state), move |state| async move {
        let (s, page_token) = match state {
            PaginationState::Start(s) => (s, None),
            PaginationState::HasMore(s, page_token) if !page_token.is_empty() => {
                (s, Some(page_token))
            }
            _ => {
                return None;
            }
        };

        let (resp, s, continuation) = match op(s, page_token).await {
            Ok(resp) => resp,
            Err(e) => return Some((Err(e), PaginationState::Done)),
        };

        let next_state = match continuation {
            Some(token) => PaginationState::HasMore(s, token),
            None => PaginationState::Done,
        };

        Some((Ok(resp), next_state))
    })
}
