use axum::routing::{delete, get, patch, post, Router};

use crate::api::catalog::*;

/// Create a new [Router] for the Delta Sharing REST API.
pub fn get_router<T: CatalogHandler + Clone>(handler: T) -> Router {
    Router::new()
        .route("/catalogs", post(create_catalog::<T>))
        .route("/catalogs", get(list_catalogs::<T>))
        .route("/catalogs/{name}", get(get_catalog::<T>))
        .route("/catalogs/{name}", patch(update_catalog::<T>))
        .route("/catalogs/{name}", delete(delete_catalog::<T>))
        .route("/schemas", post(create_schema::<T>))
        .route("/schemas", get(list_schemas::<T>))
        .route("/schemas/{full_name}", get(get_schema::<T>))
        .route("/schemas/{full_name}", delete(delete_schema::<T>))
        .route("/schemas/{full_name}", patch(update_schema::<T>))
        .with_state(handler)
}

#[cfg(any(test, feature = "integration"))]
mod integration {
    use super::*;
    use crate::models::catalog::v1::*;
    use axum::{
        body::Body,
        http::{self, Method, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    pub async fn test_catalog_router(app: Router) {
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
        assert_eq!(create_catalog_response.status(), StatusCode::OK);
        let body = create_catalog_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: CatalogInfo = serde_json::from_slice(&body).unwrap();
        assert_eq!(body.name, catalog.name);
        assert_eq!(body.comment, catalog.comment);

        // list catalogs
        let list_catalogs = Request::builder()
            .method(Method::GET)
            .uri("/catalogs")
            .body(Body::empty())
            .unwrap();
        let list_catalogs_response = app.clone().oneshot(list_catalogs).await.unwrap();
        assert_eq!(list_catalogs_response.status(), StatusCode::OK);
        let body = list_catalogs_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: ListCatalogsResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(body.catalogs.len(), 1);
        assert_eq!(body.catalogs[0].name, catalog.name);

        // get catalog
        let get_catalog = Request::builder()
            .method(Method::GET)
            .uri("/catalogs/test")
            .body(Body::empty())
            .unwrap();
        let get_catalog_response = app.clone().oneshot(get_catalog).await.unwrap();
        assert_eq!(get_catalog_response.status(), StatusCode::OK);
        let body = get_catalog_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: CatalogInfo = serde_json::from_slice(&body).unwrap();
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
        assert_eq!(update_catalog_response.status(), StatusCode::OK);
        let body = update_catalog_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: CatalogInfo = serde_json::from_slice(&body).unwrap();
        assert_eq!(body.name, new_catalog.new_name);
        assert_eq!(body.comment, new_catalog.comment);

        // get catalog again with new name
        let get_catalog = Request::builder()
            .method(Method::GET)
            .uri("/catalogs/new_test")
            .body(Body::empty())
            .unwrap();
        let get_catalog_response = app.clone().oneshot(get_catalog).await.unwrap();
        assert_eq!(get_catalog_response.status(), StatusCode::OK);
        let body = get_catalog_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: CatalogInfo = serde_json::from_slice(&body).unwrap();
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
        assert_eq!(create_schema_response.status(), StatusCode::OK);
        let body = create_schema_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: SchemaInfo = serde_json::from_slice(&body).unwrap();
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
        assert_eq!(list_schemas_response.status(), StatusCode::OK);
        let body = list_schemas_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: ListSchemasResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(body.schemas.len(), 1);
        assert_eq!(body.schemas[0].name, schema.name);

        // get schema
        let get_schema = Request::builder()
            .method(Method::GET)
            .uri("/schemas/new_test.test")
            .body(Body::empty())
            .unwrap();
        let get_schema_response = app.clone().oneshot(get_schema).await.unwrap();
        assert_eq!(get_schema_response.status(), StatusCode::OK);
        let body = get_schema_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: SchemaInfo = serde_json::from_slice(&body).unwrap();
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
        assert_eq!(update_schema_response.status(), StatusCode::OK);
        let body = update_schema_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: SchemaInfo = serde_json::from_slice(&body).unwrap();
        assert_eq!(body.name, new_schema.new_name);
        assert_eq!(body.comment, new_schema.comment);

        // delete schema
        let delete_schema = Request::builder()
            .method(Method::DELETE)
            .uri("/schemas/new_test.new_test")
            .body(Body::empty())
            .unwrap();
        let delete_schema_response = app.clone().oneshot(delete_schema).await.unwrap();
        assert_eq!(delete_schema_response.status(), StatusCode::OK);

        // assert schema is deleted
        let get_schema = Request::builder()
            .method(Method::GET)
            .uri("/schemas/new_test.new_test")
            .body(Body::empty())
            .unwrap();
        let get_schema_response = app.clone().oneshot(get_schema).await.unwrap();
        assert_eq!(get_schema_response.status(), StatusCode::NOT_FOUND);

        // delete catalog
        let delete_catalog = Request::builder()
            .method(Method::DELETE)
            .uri("/catalogs/new_test")
            .body(Body::empty())
            .unwrap();
        let delete_catalog_response = app.clone().oneshot(delete_catalog).await.unwrap();
        assert_eq!(delete_catalog_response.status(), StatusCode::OK);

        // assert catalog is deleted
        let get_catalog = Request::builder()
            .method(Method::GET)
            .uri("/catalogs/new_test")
            .body(Body::empty())
            .unwrap();
        let get_catalog_response = app.clone().oneshot(get_catalog).await.unwrap();
        assert_eq!(get_catalog_response.status(), StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::InMemoryResourceStore;
    use crate::policy::ConstantPolicy;
    use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};
    use crate::{Policy, ProvidesPolicy, ProvidesResourceStore, ResourceStore};
    use std::sync::Arc;

    #[derive(Clone)]
    struct Handler {
        store: InMemoryResourceStore,
        policy: Arc<dyn Policy>,
    }

    impl Default for Handler {
        fn default() -> Self {
            Self {
                store: InMemoryResourceStore::new(),
                policy: Arc::new(ConstantPolicy::default()),
            }
        }
    }

    impl ProvidesResourceStore for Handler {
        fn store(&self) -> &dyn ResourceStore {
            &self.store
        }
    }

    impl ProvidesPolicy for Handler {
        fn policy(&self) -> &Arc<dyn Policy> {
            &self.policy
        }
    }

    #[tokio::test]
    async fn test_catalog_router() {
        let app =
            get_router(Handler::default()).layer(AuthenticationLayer::new(AnonymousAuthenticator));
        super::integration::test_catalog_router(app).await;
    }
}
