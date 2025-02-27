use axum::http::{Method, StatusCode};
use tower::ServiceExt;

use super::super::integration::{collect_body, create_request};
use super::*;
use crate::models::credentials::v1::create_credential_request::Credential;
use crate::models::credentials::v1::*;
use crate::models::external_locations::v1::*;

pub async fn test_credentials_router(app: Router) {
    test_external_locations_router_crud(app.clone()).await;
}

async fn test_external_locations_router_crud(app: Router) {
    let credential = CreateCredentialRequest {
        name: "some_credential".to_string(),
        purpose: Purpose::Storage as i32,
        credential: Some(Credential::AzureServicePrincipal(AzureServicePrincipal {
            directory_id: "some_directory_id".to_string(),
            application_id: "some_application_id".to_string(),
            credential: Some(azure_service_principal::Credential::ClientSecret(
                "some_secret".to_string(),
            )),
        })),
        ..Default::default()
    };

    let create_credential = create_request(Method::POST, "/credentials", Some(credential.clone()));
    let create_credential_response = app.clone().oneshot(create_credential).await.unwrap();
    assert_eq!(
        create_credential_response.status(),
        StatusCode::OK,
        "create credential"
    );

    // Test Storage Location CRUD operations
    let storage_location = CreateExternalLocationRequest {
        name: "test-location".to_string(),
        url: "az://bucket".to_string(),
        credential_name: "some_credential".to_string(),
        ..Default::default()
    };

    // Create storage location
    let create_location = create_request(
        Method::POST,
        "/external-locations",
        Some(storage_location.clone()),
    );
    let create_location_response = app.clone().oneshot(create_location).await.unwrap();
    assert_eq!(
        create_location_response.status(),
        StatusCode::OK,
        "create location"
    );
    let body: ExternalLocationInfo = collect_body(create_location_response).await;
    assert_eq!(body.name, storage_location.name);

    // List storage locations
    let list_locations = create_request(Method::GET, "/external-locations", None::<()>);
    let list_locations_response = app.clone().oneshot(list_locations).await.unwrap();
    assert_eq!(
        list_locations_response.status(),
        StatusCode::OK,
        "list locations"
    );
    let body: ListExternalLocationsResponse = collect_body(list_locations_response).await;
    assert_eq!(body.external_locations.len(), 1);
    assert_eq!(body.external_locations[0].name, storage_location.name);

    // Get storage location
    let get_location = create_request(Method::GET, "/external-locations/test-location", None::<()>);
    let get_location_response = app.clone().oneshot(get_location).await.unwrap();
    assert_eq!(
        get_location_response.status(),
        StatusCode::OK,
        "get location"
    );
    let body: ExternalLocationInfo = collect_body(get_location_response).await;
    assert_eq!(body.name, storage_location.name);

    // Delete storage location
    let delete_location = create_request(
        Method::DELETE,
        "/external-locations/test-location",
        None::<()>,
    );
    let delete_location_response = app.clone().oneshot(delete_location).await.unwrap();
    assert_eq!(
        delete_location_response.status(),
        StatusCode::OK,
        "delete location"
    );

    // Verify storage location is deleted
    let get_location = create_request(Method::GET, "/external-locations/test-location", None::<()>);
    let get_location_response = app.clone().oneshot(get_location).await.unwrap();
    assert_eq!(
        get_location_response.status(),
        StatusCode::NOT_FOUND,
        "deleted location not found"
    );
}
