use cloud_client::CloudClient;
use futures::stream::BoxStream;
use futures::{Future, Stream, StreamExt, TryStreamExt};
use reqwest::IntoUrl;

use crate::api::catalogs::CatalogClient;
use crate::api::credentials::CredentialsClient;
use crate::api::external_locations::ExternalLocationsClient;
use crate::api::recipients::RecipientsClient;
use crate::api::schemas::SchemasClient;
use crate::api::shares::SharesClient;
use crate::credentials::v1::Purpose;
use crate::models::catalogs::v1 as catalog;
use crate::models::credentials::v1 as cred;
use crate::models::external_locations::v1 as loc;
use crate::models::recipients::v1 as rec;
use crate::models::schemas::v1 as schema;
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

impl SchemasClient {
    pub fn list(
        &self,
        catalog_name: impl Into<String>,
        max_results: impl Into<Option<i32>>,
    ) -> BoxStream<'_, Result<schema::SchemaInfo>> {
        let max_results = max_results.into();
        let catalog_name = catalog_name.into();
        stream_paginated(
            (catalog_name, max_results),
            move |(catalog_name, max_results), page_token| async move {
                let request = schema::ListSchemasRequest {
                    catalog_name: catalog_name.clone(),
                    max_results,
                    page_token,
                    include_browse: None,
                };
                let res = self
                    .list_schemas(&request)
                    .await
                    .map_err(|e| Error::generic(e.to_string()))?;
                Ok((
                    res.schemas,
                    (catalog_name, max_results),
                    res.next_page_token,
                ))
            },
        )
        .map_ok(|resp| futures::stream::iter(resp.into_iter().map(Ok)))
        .try_flatten()
        .boxed()
    }

    pub async fn create(
        &self,
        catalog_name: impl Into<String>,
        name: impl Into<String>,
        comment: impl Into<Option<String>>,
    ) -> Result<schema::SchemaInfo> {
        let request = schema::CreateSchemaRequest {
            catalog_name: catalog_name.into(),
            name: name.into(),
            comment: comment.into(),
            ..Default::default()
        };
        self.create_schema(&request).await
    }

    pub async fn get(
        &self,
        catalog_name: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<schema::SchemaInfo> {
        let request = schema::GetSchemaRequest {
            full_name: format!("{}.{}", catalog_name.into(), name.into()),
        };
        self.get_schema(&request).await
    }

    pub async fn delete(
        &self,
        catalog_name: impl Into<String>,
        name: impl Into<String>,
        force: impl Into<Option<bool>>,
    ) -> Result<()> {
        let request = schema::DeleteSchemaRequest {
            full_name: format!("{}.{}", catalog_name.into(), name.into()),
            force: force.into(),
        };
        tracing::info!("deleting schema {}", request.full_name);
        self.delete_schema(&request).await
    }
}

impl CredentialsClient {
    pub fn list(
        &self,
        purpose: Option<Purpose>,
        max_results: impl Into<Option<i32>>,
    ) -> BoxStream<'_, Result<cred::CredentialInfo>> {
        let max_results = max_results.into();
        let purpose = purpose.map(|p| p as i32);
        stream_paginated(max_results, move |max_results, page_token| async move {
            let request = cred::ListCredentialsRequest {
                max_results,
                page_token,
                purpose,
            };
            let res = self
                .list_credentials(&request)
                .await
                .map_err(|e| Error::generic(e.to_string()))?;
            Ok((res.credentials, max_results, res.next_page_token))
        })
        .map_ok(|resp| futures::stream::iter(resp.into_iter().map(Ok)))
        .try_flatten()
        .boxed()
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        purpose: Purpose,
        comment: impl Into<Option<String>>,
    ) -> Result<cred::CredentialInfo> {
        let request = cred::CreateCredentialRequest {
            name: name.into(),
            purpose: purpose.into(),
            comment: comment.into(),
            ..Default::default()
        };
        self.create_credential(&request).await
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<cred::CredentialInfo> {
        let request = cred::GetCredentialRequest { name: name.into() };
        self.get_credential(&request).await
    }

    pub async fn delete(&self, name: impl Into<String>) -> Result<()> {
        let request = cred::DeleteCredentialRequest { name: name.into() };
        self.delete_credential(&request).await
    }
}

impl ExternalLocationsClient {
    pub fn list(
        &self,
        max_results: impl Into<Option<i32>>,
    ) -> BoxStream<'_, Result<loc::ExternalLocationInfo>> {
        let max_results = max_results.into();
        stream_paginated(max_results, move |max_results, page_token| async move {
            let request = loc::ListExternalLocationsRequest {
                max_results,
                page_token,
                include_browse: None,
            };
            let res = self
                .list_external_locations(&request)
                .await
                .map_err(|e| Error::generic(e.to_string()))?;
            Ok((res.external_locations, max_results, res.next_page_token))
        })
        .map_ok(|resp| futures::stream::iter(resp.into_iter().map(Ok)))
        .try_flatten()
        .boxed()
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        url: impl IntoUrl,
        credential_name: impl Into<String>,
        comment: impl Into<Option<String>>,
    ) -> Result<loc::ExternalLocationInfo> {
        let request = loc::CreateExternalLocationRequest {
            name: name.into(),
            url: url
                .into_url()
                .map(|u| u.to_string())
                .map_err(|e| Error::generic(e.to_string()))?,
            credential_name: credential_name.into(),
            comment: comment.into(),
            ..Default::default()
        };
        self.create_external_location(&request).await
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<loc::ExternalLocationInfo> {
        let request = loc::GetExternalLocationRequest { name: name.into() };
        self.get_external_location(&request).await
    }

    pub async fn delete(
        &self,
        name: impl Into<String>,
        force: impl Into<Option<bool>>,
    ) -> Result<()> {
        let request = loc::DeleteExternalLocationRequest {
            name: name.into(),
            force: force.into(),
        };
        self.delete_external_location(&request).await
    }
}

impl RecipientsClient {
    pub fn list(
        &self,
        max_results: impl Into<Option<i32>>,
    ) -> BoxStream<'_, Result<rec::RecipientInfo>> {
        let max_results = max_results.into();
        stream_paginated(max_results, move |max_results, page_token| async move {
            let request = rec::ListRecipientsRequest {
                max_results,
                page_token,
            };
            let res = self
                .list_recipients(&request)
                .await
                .map_err(|e| Error::generic(e.to_string()))?;
            Ok((res.recipients, max_results, res.next_page_token))
        })
        .map_ok(|resp| futures::stream::iter(resp.into_iter().map(Ok)))
        .try_flatten()
        .boxed()
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        authentication_type: rec::AuthenticationType,
        comment: impl Into<Option<String>>,
    ) -> Result<rec::RecipientInfo> {
        let request = rec::CreateRecipientRequest {
            name: name.into(),
            authentication_type: authentication_type.into(),
            comment: comment.into(),
            ..Default::default()
        };
        self.create_recipient(&request).await
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<rec::RecipientInfo> {
        let request = rec::GetRecipientRequest { name: name.into() };
        self.get_recipient(&request).await
    }

    pub async fn delete(&self, name: impl Into<String>) -> Result<()> {
        let request = rec::DeleteRecipientRequest { name: name.into() };
        self.delete_recipient(&request).await
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
