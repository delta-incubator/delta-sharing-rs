use axum::extract::{Extension, State};
use axum::{routing::get, Json, Router};

use crate::error::{Error, Result};
use crate::models::v1::*;
use crate::{Decision, DeltaSharingHandler, Permission, Policy, Recipient, Resource};

async fn list_shares(
    State(state): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: ListSharesRequest,
) -> Result<Json<ListSharesResponse>> {
    // TODO: should we check the permission for all returned shares?
    Ok(Json(
        state.discovery.list_shares(request, &recipient).await?,
    ))
}

async fn get_share(
    State(state): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: GetShareRequest,
) -> Result<Json<GetShareResponse>> {
    check_read_share_permission(state.policy.as_ref(), &request.share, &recipient).await?;
    Ok(Json(state.discovery.get_share(request).await?))
}

async fn list_schemas(
    State(state): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: ListSchemasRequest,
) -> Result<Json<ListSchemasResponse>> {
    check_read_share_permission(state.policy.as_ref(), &request.share, &recipient).await?;
    Ok(Json(state.discovery.list_schemas(request).await?))
}

async fn list_share_tables(
    State(state): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: ListShareTablesRequest,
) -> Result<Json<ListShareTablesResponse>> {
    check_read_share_permission(state.policy.as_ref(), &request.share, &recipient).await?;
    Ok(Json(state.discovery.list_share_tables(request).await?))
}

async fn list_schema_tables(
    State(state): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: ListSchemaTablesRequest,
) -> Result<Json<ListSchemaTablesResponse>> {
    check_read_share_permission(state.policy.as_ref(), &request.share, &recipient).await?;
    Ok(Json(state.discovery.list_schema_tables(request).await?))
}

async fn check_read_share_permission(
    policy: &dyn Policy,
    share: impl Into<String>,
    recipient: &Recipient,
) -> Result<()> {
    let decision = policy
        .authorize(Resource::Share(share.into()), Permission::Read, recipient)
        .await?;
    if decision == Decision::Deny {
        return Err(Error::NotAllowed);
    }
    Ok(())
}

pub fn get_router(state: DeltaSharingHandler) -> Router {
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
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{header, HeaderValue, Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;
    use crate::policies::ConstantPolicy;
    use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};
    use crate::KernelQueryHandler;
    use crate::{
        DefaultInMemoryHandler, DeltaSharingHandler, InMemoryConfig, SchemaConfig, ShareConfig,
        TableConfig,
    };

    pub(crate) fn test_config() -> InMemoryConfig {
        InMemoryConfig {
            shares: vec![ShareConfig {
                name: "share1".to_string(),
                schema_refs: vec!["schema1".to_string()],
            }],
            schemas: vec![SchemaConfig {
                name: "schema1".to_string(),
                table_refs: vec!["table1".to_string()],
            }],
            tables: vec![TableConfig {
                name: "table1".to_string(),
                location: "file:///tmp".to_string(),
            }],
        }
    }

    pub(crate) fn test_handler() -> DefaultInMemoryHandler {
        DefaultInMemoryHandler::new(test_config())
    }

    fn get_state() -> DeltaSharingHandler {
        let discovery = Arc::new(test_handler());
        DeltaSharingHandler {
            query: KernelQueryHandler::new_background(discovery.clone(), Default::default()),
            discovery,
            policy: Arc::new(ConstantPolicy::default()),
        }
    }

    fn get_anonymous_router() -> Router {
        get_router(get_state()).layer(AuthenticationLayer::new(AnonymousAuthenticator))
    }

    #[tokio::test]
    async fn test_list_shares() {
        let app = get_anonymous_router();

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
        let app = get_anonymous_router();

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
        let app = get_anonymous_router();

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
        let app = get_anonymous_router();

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
        let app = get_anonymous_router();

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
        let app = get_anonymous_router();

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
        let app = get_anonymous_router();

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
        let app = get_anonymous_router();

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
        let app = get_anonymous_router();

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
