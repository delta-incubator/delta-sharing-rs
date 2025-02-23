use axum::routing::{delete, get, post, Router};

use crate::api::credentials::*;
use crate::api::CredentialsHandler;

pub fn get_router<T: CredentialsHandler + Clone>(handler: T) -> Router {
    Router::new()
        .route("/credentials", post(create_credential::<T>))
        .route("/credentials/{name}", get(get_credential::<T>))
        //.route("/credentials/{name}", patch(update_credential::<T>))
        .route("/credentials/{name}", delete(delete_credential::<T>))
        .route("/storage_locations", post(create_storage_location::<T>))
        .route("/storage_locations", get(list_storage_locations::<T>))
        .route("/storage_locations/{name}", get(get_storage_location::<T>))
        // .route("/storage_locations/{name}", patch(update_storage_location::<T>))
        .route(
            "/storage_locations/{name}",
            delete(delete_storage_location::<T>),
        )
        .with_state(handler)
}

#[cfg(any(test, feature = "integration"))]
pub(crate) mod integration {
    use axum::http::{Method, StatusCode};
    use tower::ServiceExt;

    use super::super::integration::{collect_body, create_request};
    use super::*;
    use crate::models::credentials::v1::*;

    pub async fn test_credentials_router(app: Router) {
        test_credentials_router_crud(app.clone()).await;
    }

    async fn test_credentials_router_crud(app: Router) {
        // Test Storage Location CRUD operations
        let storage_location = StorageLocation {
            name: "test-location".to_string(),
            url: "az://bucket".to_string(),
            r#type: StorageType::Azure as i32,
            credential: "some-cred".to_string(),
            ..Default::default()
        };

        // Create storage location
        let create_location = create_request(
            Method::POST,
            "/storage_locations",
            Some(storage_location.clone()),
        );
        let create_location_response = app.clone().oneshot(create_location).await.unwrap();
        assert_eq!(create_location_response.status(), StatusCode::OK);
        let body: StorageLocation = collect_body(create_location_response).await;
        assert_eq!(body.name, storage_location.name);

        // List storage locations
        let list_locations = create_request(Method::GET, "/storage_locations", None::<()>);
        let list_locations_response = app.clone().oneshot(list_locations).await.unwrap();
        assert_eq!(list_locations_response.status(), StatusCode::OK);
        let body: ListStorageLocationsResponse = collect_body(list_locations_response).await;
        assert_eq!(body.storage_locations.len(), 1);
        assert_eq!(body.storage_locations[0].name, storage_location.name);

        // Get storage location
        let get_location =
            create_request(Method::GET, "/storage_locations/test-location", None::<()>);
        let get_location_response = app.clone().oneshot(get_location).await.unwrap();
        assert_eq!(get_location_response.status(), StatusCode::OK);
        let body: StorageLocation = collect_body(get_location_response).await;
        assert_eq!(body.name, storage_location.name);

        // Test Credentials CRUD operations
        // let credential = Credential {
        //     name: "test-cred".to_string(),
        //     ..Default::default()
        // };

        // // Create credential
        // let create_credential =
        //     create_request(Method::POST, "/credentials", Some(credential.clone()));
        // let create_credential_response = app.clone().oneshot(create_credential).await.unwrap();
        // assert_eq!(create_credential_response.status(), StatusCode::OK);
        // let body: Credential = collect_body(create_credential_response).await;
        // assert_eq!(body.name, credential.name);

        // // Get credential
        // let get_credential = create_request(Method::GET, "/credentials/test-cred", None::<()>);
        // let get_credential_response = app.clone().oneshot(get_credential).await.unwrap();
        // assert_eq!(get_credential_response.status(), StatusCode::OK);
        // let body: Credential = collect_body(get_credential_response).await;
        // assert_eq!(body.name, credential.name);

        // // Delete credential
        // let delete_credential =
        //     create_request(Method::DELETE, "/credentials/test-cred", None::<()>);
        // let delete_credential_response = app.clone().oneshot(delete_credential).await.unwrap();
        // assert_eq!(delete_credential_response.status(), StatusCode::OK);

        // // Verify credential is deleted
        // let get_credential = create_request(Method::GET, "/credentials/test-cred", None::<()>);
        // let get_credential_response = app.clone().oneshot(get_credential).await.unwrap();
        // assert_eq!(get_credential_response.status(), StatusCode::NOT_FOUND);

        // Delete storage location
        let delete_location = create_request(
            Method::DELETE,
            "/storage_locations/test-location",
            None::<()>,
        );
        let delete_location_response = app.clone().oneshot(delete_location).await.unwrap();
        assert_eq!(delete_location_response.status(), StatusCode::OK);

        // Verify storage location is deleted
        let get_location =
            create_request(Method::GET, "/storage_locations/test-location", None::<()>);
        let get_location_response = app.clone().oneshot(get_location).await.unwrap();
        assert_eq!(get_location_response.status(), StatusCode::NOT_FOUND);
    }
}
