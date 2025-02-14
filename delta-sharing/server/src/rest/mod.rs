use delta_sharing_common::{Error, Result};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use self::router::get_router;
use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};
use crate::shutdown::shutdown_signal;
use crate::DeltaSharingHandler;

mod auth;
mod router;

pub async fn run_server(
    host: impl AsRef<str>,
    port: u16,
    handler: DeltaSharingHandler,
) -> Result<()> {
    let server = get_router(handler)
        .layer(AuthenticationLayer::new(AnonymousAuthenticator))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(format!("{}:{}", host.as_ref(), port))
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    tracing::info!("Listning on: {}", listener.local_addr().unwrap());

    axum::serve(listener, server)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{header, HeaderValue, Request, StatusCode};
    use axum::Router;
    use delta_sharing_common::models::v1::*;
    use delta_sharing_common::policies::ConstantPolicy;
    use delta_sharing_common::KernelQueryHandler;
    use delta_sharing_common::{
        DefaultInMemoryHandler, InMemoryConfig, SchemaConfig, ShareConfig, TableConfig,
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;
    use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};

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
        let result = serde_json::from_slice::<Share>(&body).unwrap();
        assert!(matches!(result, Share { .. }));
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
