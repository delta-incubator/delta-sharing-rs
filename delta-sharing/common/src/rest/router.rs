use axum::body::Body;
use axum::extract::{Extension, State};
use axum::{response::Response, routing::get, Json, Router};
use http::header::CONTENT_TYPE;

use crate::error::{Error, Result};
use crate::models::v1::*;
use crate::{Decision, DeltaSharingHandler, Permission, Policy, Recipient, Resource};

async fn list_shares(
    State(handler): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: ListSharesRequest,
) -> Result<Json<ListSharesResponse>> {
    // TODO: should we check the permission for all returned shares?
    Ok(Json(
        handler.discovery.list_shares(request, &recipient).await?,
    ))
}

async fn get_share(
    State(handler): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: GetShareRequest,
) -> Result<Json<Share>> {
    check_read_share_permission(handler.policy.as_ref(), &request.name, &recipient).await?;
    Ok(Json(handler.discovery.get_share(request).await?))
}

async fn list_schemas(
    State(handler): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: ListSchemasRequest,
) -> Result<Json<ListSchemasResponse>> {
    check_read_share_permission(handler.policy.as_ref(), &request.share, &recipient).await?;
    Ok(Json(handler.discovery.list_schemas(request).await?))
}

async fn list_share_tables(
    State(handler): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: ListShareTablesRequest,
) -> Result<Json<ListShareTablesResponse>> {
    check_read_share_permission(handler.policy.as_ref(), &request.share, &recipient).await?;
    Ok(Json(handler.discovery.list_share_tables(request).await?))
}

async fn list_schema_tables(
    State(handler): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: ListSchemaTablesRequest,
) -> Result<Json<ListSchemaTablesResponse>> {
    check_read_share_permission(handler.policy.as_ref(), &request.share, &recipient).await?;
    Ok(Json(handler.discovery.list_schema_tables(request).await?))
}

async fn get_table_version(
    State(handler): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: GetTableVersionRequest,
) -> Result<Response<Body>> {
    check_read_share_permission(handler.policy.as_ref(), &request.share, &recipient).await?;
    let result = handler.query.get_table_version(request).await?;
    let response = Response::builder()
        .header("Delta-Table-Version", result.version)
        .body(Body::empty())
        .map_err(|e| Error::generic(e.to_string()))?;
    Ok(response)
}

async fn get_table_metadata(
    State(handler): State<DeltaSharingHandler>,
    Extension(recipient): Extension<Recipient>,
    request: GetTableMetadataRequest,
) -> Result<Response<Body>> {
    check_read_share_permission(handler.policy.as_ref(), &request.share, &recipient).await?;
    let result = handler.query.get_table_metadata(request).await?;
    let response = Response::builder()
        .header(CONTENT_TYPE, "application/x-ndjson; charset=utf-8")
        .body(Body::from(query_response_to_ndjson(result)?))
        .map_err(|e| Error::generic(e.to_string()))?;
    Ok(response)
}

fn query_response_to_ndjson(response: impl IntoIterator<Item = Result<String>>) -> Result<String> {
    Ok(response
        .into_iter()
        .collect::<Result<Vec<String>>>()?
        .join("\n"))
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
        .route(
            "/shares/:share/schemas/:schema/tables/:table/version",
            get(get_table_version),
        )
        .route(
            "/shares/:share/schemas/:schema/tables/:table/metadata",
            get(get_table_metadata),
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
    use url::Url;

    use super::*;
    use crate::policies::ConstantPolicy;
    use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};
    use crate::tests::maybe_skip_dat;
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
                table_refs: vec!["table1".to_string(), "all_primitive_types".to_string()],
            }],
            tables: vec![TableConfig {
                name: "table1".to_string(),
                location: "file:///tmp".to_string(),
            }],
        }
    }

    pub(crate) fn test_config_dat() -> InMemoryConfig {
        let dat_dir = testutils::dat::find_dat_dir().unwrap();
        let case_root = dat_dir.join("out/reader_tests/generated");
        let entries = std::fs::read_dir(case_root)
            .unwrap()
            .map(|res| {
                res.map(|e| {
                    (
                        e.file_name().to_str().unwrap().to_string(),
                        Url::from_directory_path(e.path().join("delta"))
                            .unwrap()
                            .to_string(),
                    )
                })
            })
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        InMemoryConfig {
            shares: vec![ShareConfig {
                name: "dat".to_string(),
                schema_refs: vec!["reader_tests".to_string()],
            }],
            schemas: vec![SchemaConfig {
                name: "reader_tests".to_string(),
                table_refs: entries.iter().cloned().map(|t| t.0).collect(),
            }],
            tables: entries
                .iter()
                .cloned()
                .map(|t| TableConfig {
                    name: t.0,
                    location: t.1,
                })
                .collect(),
        }
    }

    pub(crate) fn test_handler() -> DefaultInMemoryHandler {
        DefaultInMemoryHandler::new(test_config())
    }

    pub(crate) fn test_handler_dat() -> DefaultInMemoryHandler {
        DefaultInMemoryHandler::new(test_config_dat())
    }

    fn get_state() -> DeltaSharingHandler {
        let discovery = Arc::new(test_handler());
        DeltaSharingHandler {
            query: KernelQueryHandler::new_background(discovery.clone(), Default::default()),
            discovery,
            policy: Arc::new(ConstantPolicy::default()),
        }
    }

    fn get_state_dat() -> DeltaSharingHandler {
        let discovery = Arc::new(test_handler_dat());
        DeltaSharingHandler {
            query: KernelQueryHandler::new_background(discovery.clone(), Default::default()),
            discovery,
            policy: Arc::new(ConstantPolicy::default()),
        }
    }

    fn get_anonymous_router() -> Router {
        get_router(get_state()).layer(AuthenticationLayer::new(AnonymousAuthenticator))
    }

    fn get_anonymous_router_dat() -> Router {
        get_router(get_state_dat()).layer(AuthenticationLayer::new(AnonymousAuthenticator))
    }

    fn get_test_request(uri: impl AsRef<str>) -> Request<Body> {
        Request::builder()
            .uri(uri.as_ref())
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str("Bearer token").unwrap(),
            )
            .body(Body::empty())
            .unwrap()
    }

    #[tokio::test]
    async fn test_list_shares() {
        let app = get_anonymous_router();
        let request = get_test_request("/shares");

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<ListSharesResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_get_share() {
        let app = get_anonymous_router();
        let request = get_test_request("/shares/share1");

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<Share>(&body).unwrap();
        assert!(matches!(result, Share { .. }));
    }

    #[tokio::test]
    async fn test_get_share_not_found() {
        let app = get_anonymous_router();
        let request = get_test_request("/shares/nonexistent");

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_schemas() {
        let app = get_anonymous_router();
        let request = get_test_request("/shares/share1/schemas");

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<ListSchemasResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_list_schemas_not_found() {
        let app = get_anonymous_router();
        let request = get_test_request("/shares/nonexistent/schemas");

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_share_tables() {
        let app = get_anonymous_router();
        let request = get_test_request("/shares/share1/all-tables");

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<ListShareTablesResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_list_share_tables_not_found() {
        let app = get_anonymous_router();
        let request = get_test_request("/shares/nonexistent/all-tables");

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_schema_tables() {
        let app = get_anonymous_router();
        let request = get_test_request("/shares/share1/schemas/schema1/tables");

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result = serde_json::from_slice::<ListSchemaTablesResponse>(&body).unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_list_schema_tables_not_found() {
        let app = get_anonymous_router();
        let request = get_test_request("/shares/share1/schemas/nonexistent/tables");

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_table_version() {
        maybe_skip_dat!();

        let app = get_anonymous_router_dat();
        let cases = [
            ("all_primitive_types", Some("0")),
            ("basic_append", Some("1")),
            ("basic_partitioned", Some("1")),
        ];

        for (name, expected) in cases {
            let request = get_test_request(format!(
                "/shares/dat/schemas/reader_tests/tables/{}/version",
                name
            ));
            let response = app.clone().oneshot(request).await.unwrap();
            assert!(response.status().is_success());
            let maybe_version = response
                .headers()
                .get("Delta-Table-Version")
                .map(|v| v.to_str().unwrap());
            assert_eq!(maybe_version, expected);
        }
    }

    #[tokio::test]
    async fn test_get_table_metadata() {
        maybe_skip_dat!();

        let app = get_anonymous_router_dat();
        let cases = ["all_primitive_types", "basic_append", "basic_partitioned"];

        for name in cases {
            let request = get_test_request(format!(
                "/shares/dat/schemas/reader_tests/tables/{}/metadata",
                name
            ));
            let response = app.clone().oneshot(request).await.unwrap();
            assert!(response.status().is_success());
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let content = String::from_utf8_lossy(&body);
            for line in content.split("\n") {
                let _: ParquetLogMessage = serde_json::from_str(line).unwrap();
            }
        }
    }
}
