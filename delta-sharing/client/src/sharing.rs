use std::sync::Arc;

use futures::{Stream, TryStreamExt};

use crate::service::ServiceClient;
use cloud_client::stream_paginated;
use delta_sharing_common::models::sharing::v1::*;
use delta_sharing_common::Result;

pub struct DeltaSharingClient {
    client: Arc<dyn ServiceClient>,
}

impl DeltaSharingClient {
    pub fn try_new(client: Arc<dyn ServiceClient>) -> Result<Self> {
        Ok(Self { client })
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
            self.client.list_shares(request).await.map(|mut resp| {
                let max_results = max_results
                    .map(|m| m - resp.items.len() as i32)
                    .filter(|m| *m > 0);
                let page_token = resp.next_page_token.take();
                (resp, max_results, page_token)
            })
        })
        .map_ok(|resp| futures::stream::iter(resp.items.into_iter().map(Ok)))
        .try_flatten()
    }

    pub async fn list_schema_tables(
        &self,
        share: impl Into<String>,
        schema: impl Into<String>,
        max_results: Option<i32>,
    ) -> impl Stream<Item = Result<SharingTable>> + '_ {
        stream_paginated(
            (share.into(), schema.into(), max_results),
            move |(share, schema, max_results), page_token| async move {
                let req = ListSchemaTablesRequest {
                    share: share.clone(),
                    name: schema.clone(),
                    max_results,
                    page_token,
                };
                let mut resp = self.client.list_schema_tables(req).await?;
                let max_results = max_results
                    .map(|m| m - resp.items.len() as i32)
                    .filter(|m| *m > 0);
                let page_token = resp.next_page_token.take();
                Ok((resp, (share, schema, max_results), page_token))
            },
        )
        .map_ok(|resp| futures::stream::iter(resp.items.into_iter().map(Ok)))
        .try_flatten()
    }

    pub async fn list_share_tables(
        &self,
        share: impl Into<String>,
        max_results: Option<i32>,
    ) -> impl Stream<Item = Result<SharingTable>> + '_ {
        stream_paginated(
            (share.into(), max_results),
            move |(share, max_results), page_token| async move {
                let req = ListShareTablesRequest {
                    name: share.clone(),
                    max_results,
                    page_token,
                };
                let mut resp = self.client.list_share_tables(req).await?;
                let max_results = max_results
                    .map(|m| m - resp.items.len() as i32)
                    .filter(|m| *m > 0);
                let page_token = resp.next_page_token.take();
                Ok((resp, (share, max_results), page_token))
            },
        )
        .map_ok(|resp| futures::stream::iter(resp.items.into_iter().map(Ok)))
        .try_flatten()
    }
}
