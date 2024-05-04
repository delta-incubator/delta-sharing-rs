use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::{routing::get, Json, Router};
use axum_extra::{headers, headers::authorization::Bearer, TypedHeader};
use delta_sharing_core::{
    DiscoveryHandler, GetShareRequest, GetShareResponse, ListSchemaTablesRequest,
    ListSchemaTablesResponse, ListSchemasRequest, ListSchemasResponse, ListShareTablesRequest,
    ListShareTablesResponse, ListSharesRequest, ListSharesResponse, RecipientHandler,
};
use serde::Deserialize;

use crate::error::Result;

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
) -> Result<Json<ListSharesResponse>> {
    let request = ListSharesRequest {
        max_results: pagination.0.max_results,
        page_token: pagination.0.page_token,
    };

    let recipient = state
        .auth
        .get_recipient(autorization.map(|a| a.0.token().to_string()))
        .await?;

    let response = state.discovery.list_shares(request, recipient).await?;

    Ok(Json(response))
}

async fn get_share<T: Send>(
    State(state): State<DeltaSharingState<T>>,
    autorization: Option<TypedHeader<headers::Authorization<Bearer>>>,
    Path(share): Path<String>,
) -> Result<Json<GetShareResponse>> {
    let request = GetShareRequest {
        share: share.to_ascii_lowercase(),
    };

    let recipient = state
        .auth
        .get_recipient(autorization.map(|a| a.0.token().to_string()))
        .await?;

    let response = state.discovery.get_share(request, recipient).await?;

    Ok(Json(response))
}

async fn list_schemas<T: Send>(
    State(state): State<DeltaSharingState<T>>,
    autorization: Option<TypedHeader<headers::Authorization<Bearer>>>,
    pagination: Query<Pagination>,
    Path(share): Path<String>,
) -> Result<Json<ListSchemasResponse>> {
    let request = ListSchemasRequest {
        max_results: pagination.0.max_results,
        page_token: pagination.0.page_token,
        share: share.to_ascii_lowercase(),
    };

    let recipient = state
        .auth
        .get_recipient(autorization.map(|a| a.0.token().to_string()))
        .await?;

    let response = state.discovery.list_schemas(request, recipient).await?;

    Ok(Json(response))
}

async fn list_share_tables<T: Send>(
    State(state): State<DeltaSharingState<T>>,
    autorization: Option<TypedHeader<headers::Authorization<Bearer>>>,
    pagination: Query<Pagination>,
    Path(share): Path<String>,
) -> Result<Json<ListShareTablesResponse>> {
    let request = ListShareTablesRequest {
        max_results: pagination.0.max_results,
        page_token: pagination.0.page_token,
        share: share.to_ascii_lowercase(),
    };

    let recipient = state
        .auth
        .get_recipient(autorization.map(|a| a.0.token().to_string()))
        .await?;

    let response = state
        .discovery
        .list_share_tables(request, recipient)
        .await?;

    Ok(Json(response))
}

async fn list_schema_tables<T: Send>(
    State(state): State<DeltaSharingState<T>>,
    autorization: Option<TypedHeader<headers::Authorization<Bearer>>>,
    pagination: Query<Pagination>,
    Path((share, schema)): Path<(String, String)>,
) -> Result<Json<ListSchemaTablesResponse>> {
    let request = ListSchemaTablesRequest {
        max_results: pagination.0.max_results,
        page_token: pagination.0.page_token,
        share: share.to_ascii_lowercase(),
        schema: schema.to_ascii_lowercase(),
    };

    let recipient = state
        .auth
        .get_recipient(autorization.map(|a| a.0.token().to_string()))
        .await?;

    let response = state
        .discovery
        .list_schema_tables(request, recipient)
        .await?;

    Ok(Json(response))
}

pub fn get_router<T: Send + Clone + 'static>(state: DeltaSharingState<T>) -> Router {
    Router::new()
        .route("/shares", get(list_shares))
        .route("/shares/:share", get(get_share))
        .route("/shares/:share/schemas", get(list_schemas))
        .route("/shares/:share/all-tables", get(list_share_tables))
        .route(
            "/shares/:share/schemas/:schema/tables",
            get(list_schema_tables),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{header, HeaderValue, Request, StatusCode};
    use delta_sharing_core::handlers::VoidRecipientHandler;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;
    use crate::tests::test_handler;

    fn get_state() -> DeltaSharingState<()> {
        DeltaSharingState {
            discovery: Arc::new(test_handler()),
            auth: Arc::new(VoidRecipientHandler {}),
        }
    }

    #[tokio::test]
    async fn test_list_shares() {
        let app = get_router(get_state());

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
        let app = get_router(get_state());

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

    #[tokio::test]
    async fn test_get_share_not_found() {
        let app = get_router(get_state());

        let request: Request<Body> = Request::builder()
            .uri("/shares/nonexistent")
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_schemas() {
        let app = get_router(get_state());

        let request = Request::builder()
            .uri("/shares/share1/schemas")
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<ListSchemasResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_list_schemas_not_found() {
        let app = get_router(get_state());

        let request: Request<Body> = Request::builder()
            .uri("/shares/nonexistent/schemas")
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_share_tables() {
        let app = get_router(get_state());

        let request = Request::builder()
            .uri("/shares/share1/all-tables")
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<ListShareTablesResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_list_share_tables_not_found() {
        let app = get_router(get_state());

        let request: Request<Body> = Request::builder()
            .uri("/shares/nonexistent/all-tables")
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_schema_tables() {
        let app = get_router(get_state());

        let request = Request::builder()
            .uri("/shares/share1/schemas/schema1/tables")
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<ListSchemaTablesResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_list_schema_tables_not_found() {
        let app = get_router(get_state());

        let request: Request<Body> = Request::builder()
            .uri("/shares/share1/schemas/nonexistent/tables")
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
