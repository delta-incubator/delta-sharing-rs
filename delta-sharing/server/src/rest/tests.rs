use axum::body::Body;
use axum::Router;
use delta_sharing_common::models::v1::*;
use delta_sharing_common::rest::get_sharing_router;
use delta_sharing_common::Result;

use std::sync::Arc;

use axum::http::{header, HeaderValue, Request, StatusCode};
use delta_sharing_common::policies::ConstantPolicy;
use delta_sharing_common::KernelQueryHandler;
use delta_sharing_common::{
    DefaultInMemoryHandler, InMemoryConfig, SchemaConfig, ShareConfig, TableConfig,
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use url::Url;

use crate::handler::DeltaSharingHandler;
use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};
use crate::tests::maybe_skip_dat;

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
    get_sharing_router(get_state()).layer(AuthenticationLayer::new(AnonymousAuthenticator))
}

fn get_anonymous_router_dat() -> Router {
    get_sharing_router(get_state_dat()).layer(AuthenticationLayer::new(AnonymousAuthenticator))
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
