use super::*;
use crate::models::catalogs::v1::*;
use crate::models::schemas::v1::*;
use axum::{
    body::Body,
    http::{self, Method, Request, StatusCode},
};
use tower::ServiceExt;

// TODO: test un-happy paths ... missing fields, etc
//
pub async fn test_catalog_router(app: Router) {
    test_catalog_router_crud(app.clone()).await;
    test_catalog_router_list(app.clone()).await;
}

// validate that the catalog router can list catalogs and works with page / limits
async fn test_catalog_router_list(app: Router) {
    let list_catalogs = create_request(Method::GET, "/catalogs", None::<()>);
    let list_catalogs_response = app.clone().oneshot(list_catalogs).await.unwrap();
    assert_eq!(list_catalogs_response.status(), StatusCode::OK);
    let body: ListCatalogsResponse = collect_body(list_catalogs_response).await;
    assert_eq!(
        body.catalogs.len(),
        0,
        "expected no catalogs on initial list"
    );

    // create a catalog
    let catalog = CatalogInfo {
        name: "test".to_string(),
        comment: Some("test catalog".to_string()),
        ..Default::default()
    };
    let create_catalog = create_request(Method::POST, "/catalogs", Some(catalog));
    let create_catalog_response = app.clone().oneshot(create_catalog).await.unwrap();
    assert_eq!(
        create_catalog_response.status(),
        StatusCode::OK,
        "create catalog"
    );

    // list catalogs
    let list_catalogs = create_request(Method::GET, "/catalogs", None::<()>);
    let list_catalogs_response = app.clone().oneshot(list_catalogs).await.unwrap();
    assert_eq!(
        list_catalogs_response.status(),
        StatusCode::OK,
        "list catalogs"
    );
    let body: ListCatalogsResponse = collect_body(list_catalogs_response).await;
    assert_eq!(body.catalogs.len(), 1);

    // create a schema
    let schema = SchemaInfo {
        name: "test".to_string(),
        catalog_name: "test".to_string(),
        comment: Some("test schema".to_string()),
        ..Default::default()
    };
    let create_schema = create_request(Method::POST, "/schemas", Some(schema));
    let create_schema_response = app.clone().oneshot(create_schema).await.unwrap();
    assert_eq!(
        create_schema_response.status(),
        StatusCode::OK,
        "create schema"
    );

    // list schemas
    let list_schemas = create_request(Method::GET, "/schemas?catalog_name=test", None::<()>);
    let list_schemas_response = app.clone().oneshot(list_schemas).await.unwrap();
    assert_eq!(
        list_schemas_response.status(),
        StatusCode::OK,
        "list schemas"
    );
    let body: ListSchemasResponse = collect_body(list_schemas_response).await;
    assert_eq!(body.schemas.len(), 1);

    // create some more schemas
    let schema = SchemaInfo {
        name: "test2".to_string(),
        catalog_name: "test".to_string(),
        comment: Some("test schema".to_string()),
        ..Default::default()
    };
    let create_schema = create_request(Method::POST, "/schemas", Some(schema));
    let create_schema_response = app.clone().oneshot(create_schema).await.unwrap();
    assert_eq!(create_schema_response.status(), StatusCode::OK);

    let schema = SchemaInfo {
        name: "test3".to_string(),
        catalog_name: "test".to_string(),
        comment: Some("test schema".to_string()),
        ..Default::default()
    };
    let create_schema = create_request(Method::POST, "/schemas", Some(schema));
    let create_schema_response = app.clone().oneshot(create_schema).await.unwrap();
    assert_eq!(create_schema_response.status(), StatusCode::OK);

    // list schemas
    let list_schemas = create_request(Method::GET, "/schemas?catalog_name=test", None::<()>);
    let list_schemas_response = app.clone().oneshot(list_schemas).await.unwrap();
    assert_eq!(
        list_schemas_response.status(),
        StatusCode::OK,
        "list schemas"
    );
    let body: ListSchemasResponse = collect_body(list_schemas_response).await;
    assert_eq!(body.schemas.len(), 3);

    // list schemas with limit
    let list_schemas = create_request(
        Method::GET,
        "/schemas?catalog_name=test&max_results=2",
        None::<()>,
    );
    let list_schemas_response = app.clone().oneshot(list_schemas).await.unwrap();
    assert_eq!(
        list_schemas_response.status(),
        StatusCode::OK,
        "list schemas with limit"
    );
    let body: ListSchemasResponse = collect_body(list_schemas_response).await;
    assert_eq!(body.schemas.len(), 2);
    let next_page_token = body.next_page_token.unwrap();

    // list schemas with limit and page
    let list_schemas = create_request(
        Method::GET,
        &format!(
            "/schemas?catalog_name=test&max_results=2&page_token={}",
            next_page_token
        ),
        None::<()>,
    );
    let list_schemas_response = app.clone().oneshot(list_schemas).await.unwrap();
    assert_eq!(
        list_schemas_response.status(),
        StatusCode::OK,
        "list schemas with page"
    );
    let body: ListSchemasResponse = collect_body(list_schemas_response).await;
    assert_eq!(body.schemas.len(), 1);
}

async fn test_catalog_router_crud(app: Router) {
    let catalog = CatalogInfo {
        name: "test".to_string(),
        comment: Some("test catalog".to_string()),
        ..Default::default()
    };

    // create a catalog
    let create_catalog = Request::builder()
        .method(Method::POST)
        .uri("/catalogs")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(serde_json::to_vec(&catalog).unwrap()))
        .unwrap();
    let create_catalog_response = app.clone().oneshot(create_catalog).await.unwrap();
    assert_eq!(
        create_catalog_response.status(),
        StatusCode::OK,
        "create catalog"
    );
    let body: CatalogInfo = collect_body(create_catalog_response).await;
    assert_eq!(body.name, catalog.name);
    assert_eq!(body.comment, catalog.comment);

    // list catalogs
    let list_catalogs = Request::builder()
        .method(Method::GET)
        .uri("/catalogs")
        .body(Body::empty())
        .unwrap();
    let list_catalogs_response = app.clone().oneshot(list_catalogs).await.unwrap();
    assert_eq!(
        list_catalogs_response.status(),
        StatusCode::OK,
        "list catalogs"
    );
    let body: ListCatalogsResponse = collect_body(list_catalogs_response).await;
    assert_eq!(body.catalogs.len(), 1);
    assert_eq!(body.catalogs[0].name, catalog.name);

    // get catalog
    let get_catalog = Request::builder()
        .method(Method::GET)
        .uri("/catalogs/test")
        .body(Body::empty())
        .unwrap();
    let get_catalog_response = app.clone().oneshot(get_catalog).await.unwrap();
    assert_eq!(get_catalog_response.status(), StatusCode::OK, "get catalog");
    let body: CatalogInfo = collect_body(get_catalog_response).await;
    assert_eq!(body.name, catalog.name);

    // update catalog
    let new_catalog = UpdateCatalogRequest {
        name: "test".to_string(),
        new_name: "new_test".to_string(),
        comment: Some("new comment".to_string()),
        ..Default::default()
    };
    let update_catalog = Request::builder()
        .method(Method::PATCH)
        .uri("/catalogs/test")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(serde_json::to_vec(&new_catalog).unwrap()))
        .unwrap();
    let update_catalog_response = app.clone().oneshot(update_catalog).await.unwrap();
    assert_eq!(
        update_catalog_response.status(),
        StatusCode::OK,
        "update catalog"
    );
    let body: CatalogInfo = collect_body(update_catalog_response).await;
    assert_eq!(body.name, new_catalog.new_name);
    assert_eq!(body.comment, new_catalog.comment);

    // get catalog again with new name
    let get_catalog = Request::builder()
        .method(Method::GET)
        .uri("/catalogs/new_test")
        .body(Body::empty())
        .unwrap();
    let get_catalog_response = app.clone().oneshot(get_catalog).await.unwrap();
    assert_eq!(
        get_catalog_response.status(),
        StatusCode::OK,
        "get updated catalog"
    );
    let body: CatalogInfo = collect_body(get_catalog_response).await;
    assert_eq!(body.name, new_catalog.new_name);

    // create a schema
    let schema = SchemaInfo {
        name: "test".to_string(),
        catalog_name: "new_test".to_string(),
        comment: Some("test schema".to_string()),
        ..Default::default()
    };
    let create_schema = Request::builder()
        .method(Method::POST)
        .uri("/schemas")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(serde_json::to_vec(&schema).unwrap()))
        .unwrap();
    let create_schema_response = app.clone().oneshot(create_schema).await.unwrap();
    assert_eq!(
        create_schema_response.status(),
        StatusCode::OK,
        "create schema"
    );
    let body: SchemaInfo = collect_body(create_schema_response).await;
    assert_eq!(body.name, schema.name);
    assert_eq!(body.catalog_name, schema.catalog_name);
    assert_eq!(body.comment, schema.comment);

    // list schemas
    let list_schemas = Request::builder()
        .method(Method::GET)
        .uri("/schemas?catalog_name=new_test")
        .body(Body::empty())
        .unwrap();
    let list_schemas_response = app.clone().oneshot(list_schemas).await.unwrap();
    assert_eq!(
        list_schemas_response.status(),
        StatusCode::OK,
        "list schemas"
    );
    let body: ListSchemasResponse = collect_body(list_schemas_response).await;
    assert_eq!(body.schemas.len(), 1);
    assert_eq!(body.schemas[0].name, schema.name);

    // get schema
    let get_schema = Request::builder()
        .method(Method::GET)
        .uri("/schemas/new_test.test")
        .body(Body::empty())
        .unwrap();
    let get_schema_response = app.clone().oneshot(get_schema).await.unwrap();
    assert_eq!(get_schema_response.status(), StatusCode::OK, "get schema");
    let body: SchemaInfo = collect_body(get_schema_response).await;
    assert_eq!(body.name, schema.name);

    // update schema
    let new_schema = UpdateSchemaRequest {
        full_name: "new_test.test".to_string(),
        new_name: "new_test".to_string(),
        comment: Some("new comment".to_string()),
        ..Default::default()
    };
    let update_schema = Request::builder()
        .method(Method::PATCH)
        .uri("/schemas/new_test.test")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(serde_json::to_vec(&new_schema).unwrap()))
        .unwrap();
    let update_schema_response = app.clone().oneshot(update_schema).await.unwrap();
    assert_eq!(
        update_schema_response.status(),
        StatusCode::OK,
        "update schema"
    );
    let body: SchemaInfo = collect_body(update_schema_response).await;
    assert_eq!(body.name, new_schema.new_name);
    assert_eq!(body.comment, new_schema.comment);

    // delete schema
    let delete_schema = Request::builder()
        .method(Method::DELETE)
        .uri("/schemas/new_test.new_test")
        .body(Body::empty())
        .unwrap();
    let delete_schema_response = app.clone().oneshot(delete_schema).await.unwrap();
    assert_eq!(
        delete_schema_response.status(),
        StatusCode::OK,
        "delete schema"
    );

    // assert schema is deleted
    let get_schema = Request::builder()
        .method(Method::GET)
        .uri("/schemas/new_test.new_test")
        .body(Body::empty())
        .unwrap();
    let get_schema_response = app.clone().oneshot(get_schema).await.unwrap();
    assert_eq!(
        get_schema_response.status(),
        StatusCode::NOT_FOUND,
        "schema not found"
    );

    // delete catalog
    let delete_catalog = Request::builder()
        .method(Method::DELETE)
        .uri("/catalogs/new_test")
        .body(Body::empty())
        .unwrap();
    let delete_catalog_response = app.clone().oneshot(delete_catalog).await.unwrap();
    assert_eq!(
        delete_catalog_response.status(),
        StatusCode::OK,
        "delete catalog"
    );

    // assert catalog is deleted
    let get_catalog = Request::builder()
        .method(Method::GET)
        .uri("/catalogs/new_test")
        .body(Body::empty())
        .unwrap();
    let get_catalog_response = app.clone().oneshot(get_catalog).await.unwrap();
    assert_eq!(
        get_catalog_response.status(),
        StatusCode::NOT_FOUND,
        "catalog not found"
    );
}
