use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::{routing::get, Json, Router};
use delta_sharing_core::policies::{Decision, Permission, Policy, Resource};
use delta_sharing_core::types as t;
use delta_sharing_core::{DiscoveryHandler, Error as CoreError, TableQueryHandler};
use serde::Deserialize;

use crate::error::Result;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pagination {
    max_results: Option<i32>,
    page_token: Option<String>,
}

#[derive(Clone)]
pub struct DeltaSharingState<T: Send + Sync> {
    pub discovery: Arc<dyn DiscoveryHandler<Recipient = T>>,
    pub query: Arc<dyn TableQueryHandler>,
    pub policy: Arc<dyn Policy<Recipient = T>>,
}

async fn list_shares<T: Send + Sync>(
    State(state): State<DeltaSharingState<T>>,
    Extension(recipient): Extension<T>,
    pagination: Query<Pagination>,
) -> Result<Json<t::ListSharesResponse>> {
    let request = t::ListSharesRequest {
        max_results: pagination.0.max_results,
        page_token: pagination.0.page_token,
    };
    // TODO: should we check the permission for all returned shares?
    Ok(Json(state.discovery.list_shares(request, recipient).await?))
}

async fn get_share<T: Send + Sync>(
    State(state): State<DeltaSharingState<T>>,
    Extension(recipient): Extension<T>,
    Path(share): Path<String>,
) -> Result<Json<t::GetShareResponse>> {
    let request = t::GetShareRequest {
        share: share.to_ascii_lowercase(),
    };
    check_read_share_permission(state.policy.as_ref(), share, &recipient).await?;
    Ok(Json(state.discovery.get_share(request).await?))
}

async fn list_schemas<T: Send + Sync>(
    State(state): State<DeltaSharingState<T>>,
    Extension(recipient): Extension<T>,
    pagination: Query<Pagination>,
    Path(share): Path<String>,
) -> Result<Json<t::ListSchemasResponse>> {
    let request = t::ListSchemasRequest {
        max_results: pagination.0.max_results,
        page_token: pagination.0.page_token,
        share: share.to_ascii_lowercase(),
    };
    check_read_share_permission(state.policy.as_ref(), share, &recipient).await?;
    Ok(Json(state.discovery.list_schemas(request).await?))
}

async fn list_share_tables<T: Send + Sync>(
    State(state): State<DeltaSharingState<T>>,
    Extension(recipient): Extension<T>,
    pagination: Query<Pagination>,
    Path(share): Path<String>,
) -> Result<Json<t::ListShareTablesResponse>> {
    let request = t::ListShareTablesRequest {
        max_results: pagination.0.max_results,
        page_token: pagination.0.page_token,
        share: share.to_ascii_lowercase(),
    };
    check_read_share_permission(state.policy.as_ref(), share, &recipient).await?;
    Ok(Json(state.discovery.list_share_tables(request).await?))
}

async fn list_schema_tables<T: Send + Sync>(
    State(state): State<DeltaSharingState<T>>,
    Extension(recipient): Extension<T>,
    pagination: Query<Pagination>,
    Path((share, schema)): Path<(String, String)>,
) -> Result<Json<t::ListSchemaTablesResponse>> {
    let request = t::ListSchemaTablesRequest {
        max_results: pagination.0.max_results,
        page_token: pagination.0.page_token,
        share: share.to_ascii_lowercase(),
        schema: schema.to_ascii_lowercase(),
    };
    check_read_share_permission(state.policy.as_ref(), share, &recipient).await?;
    Ok(Json(state.discovery.list_schema_tables(request).await?))
}

async fn check_read_share_permission<T: Send + Sync>(
    policy: &dyn Policy<Recipient = T>,
    share: String,
    recipient: &T,
) -> Result<()> {
    let decision = policy
        .authorize(Resource::Share(share), Permission::Read, recipient)
        .await?;
    if decision == Decision::Deny {
        return Err(CoreError::NotAllowed.into());
    }
    Ok(())
}

pub fn get_router<T: Send + Sync + Clone + 'static>(state: DeltaSharingState<T>) -> Router {
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
    use delta_sharing_core::policies::{ConstantPolicy, RecipientId};
    use delta_sharing_core::query::KernelQueryHandler;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;
    use crate::auth::{AnonymousAuthenticator, AuthorizationLayer};
    use crate::tests::test_handler;

    fn get_state() -> DeltaSharingState<RecipientId> {
        let discovery = Arc::new(test_handler());
        DeltaSharingState {
            query: KernelQueryHandler::new_background(discovery.clone(), Default::default()),
            discovery,
            policy: Arc::new(ConstantPolicy::<RecipientId>::default()),
        }
    }

    fn get_anoymous_router() -> Router {
        get_router(get_state()).layer(AuthorizationLayer::new(Arc::new(AnonymousAuthenticator)))
    }

    #[tokio::test]
    async fn test_list_shares() {
        let app = get_anoymous_router();

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
        let result = serde_json::from_slice::<t::ListSharesResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_get_share() {
        let app = get_anoymous_router();

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
        let result = serde_json::from_slice::<t::GetShareResponse>(&body).unwrap();
        assert!(matches!(result, t::GetShareResponse { share: Some(_) }));
    }

    #[tokio::test]
    async fn test_get_share_not_found() {
        let app = get_anoymous_router();

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
        let app = get_anoymous_router();

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
        let result = serde_json::from_slice::<t::ListSchemasResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_list_schemas_not_found() {
        let app = get_anoymous_router();

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
        let app = get_anoymous_router();

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
        let result = serde_json::from_slice::<t::ListShareTablesResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_list_share_tables_not_found() {
        let app = get_anoymous_router();

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
        let app = get_anoymous_router();

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
        let result = serde_json::from_slice::<t::ListSchemaTablesResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_list_schema_tables_not_found() {
        let app = get_anoymous_router();

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
