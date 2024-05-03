use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::{routing::get, Json, Router};
use axum_extra::{headers, headers::authorization::Bearer, TypedHeader};
use delta_sharing_core::{
    DiscoveryHandler, GetShareRequest, GetShareResponse, ListSharesRequest, ListSharesResponse,
    RecipientHandler,
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
    pub discovery: Arc<dyn DiscoveryHandler<Recipient = T>>,
    pub auth: Arc<dyn RecipientHandler<Recipient = T>>,
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

async fn get_share<T: Send>(
    State(state): State<DeltaSharingState<T>>,
    autorization: Option<TypedHeader<headers::Authorization<Bearer>>>,
    Path(share): Path<String>,
) -> Json<GetShareResponse> {
    let request = GetShareRequest {
        share: share.to_ascii_lowercase(),
    };

    let recipient = state
        .auth
        .get_recipient(autorization.map(|a| a.0.token().to_string()))
        .await
        .unwrap();

    let response = state.discovery.get_share(request, recipient).await.unwrap();

    Json(response)
}

pub fn get_router<T: Send + Clone + 'static>(state: DeltaSharingState<T>) -> Router {
    Router::new()
        .route("/shares", get(list_shares))
        .route("/shares/:share", get(get_share))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{header, HeaderValue, Request};
    use delta_sharing_core::handlers::VoidRecipientHandler;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;
    use crate::tests::test_handler;

    #[tokio::test]
    async fn test_list_shares() {
        let state = DeltaSharingState {
            discovery: Arc::new(test_handler()),
            auth: Arc::new(VoidRecipientHandler {}),
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

    #[tokio::test]
    async fn test_get_share() {
        let state = DeltaSharingState {
            discovery: Arc::new(test_handler()),
            auth: Arc::new(VoidRecipientHandler {}),
        };
        let app = get_router(state);

        let request = Request::builder()
            .uri("/shares/share1")
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<GetShareResponse>(&body).unwrap();
        assert!(matches!(result, GetShareResponse { share: Some(_) }));
    }
}
