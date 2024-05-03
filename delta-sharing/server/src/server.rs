use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    routing::{get, post},
    Json, Router,
};
use axum_extra::{headers, headers::authorization::Bearer, TypedHeader};
use delta_sharing_core::Error;
use delta_sharing_core::{
    DiscoveryHandler, ListSharesRequest, ListSharesResponse, RecipientHandler,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pagination {
    max_results: Option<i32>,
    page_token: Option<String>,
}

#[derive(Clone)]
pub struct DeltaSharingState<T: Send> {
    discovery: Arc<dyn DiscoveryHandler<Recipient = T>>,
    auth: Arc<dyn RecipientHandler<Recipient = T>>,
}

async fn list_shares<T: Send>(
    State(state): State<DeltaSharingState<T>>,
    autorization: Option<TypedHeader<headers::Authorization<Bearer>>>,
    pagination: Query<Pagination>,
) -> Json<ListSharesResponse> {
    let request = ListSharesRequest {
        max_results: pagination.0.max_results,
        page_token: pagination.0.page_token,
    };

    let recipient = state
        .auth
        .get_recipient(autorization.map(|a| a.0.token().to_string()))
        .await
        .unwrap();

    let response = state
        .discovery
        .list_shares(request, recipient)
        .await
        .unwrap();

    Json(response)
}

fn get_router<T: Send + Clone + 'static>(state: DeltaSharingState<T>) -> Router {
    Router::new()
        .route("/shares", get(list_shares))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{HeaderValue, Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;
    use crate::tests::test_handler;

    #[derive(Clone)]
    struct MockRecipientHandler {}

    #[async_trait::async_trait]
    impl RecipientHandler for MockRecipientHandler {
        type Recipient = ();

        async fn get_recipient(&self, _authorization: Option<String>) -> Result<(), Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_list_shares() {
        let state = DeltaSharingState {
            discovery: Arc::new(test_handler()),
            auth: Arc::new(MockRecipientHandler {}),
        };
        let app = get_router(state);

        let request = Request::builder()
            .uri("/shares")
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<ListSharesResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }
}
