use delta_sharing_common::models::catalogs::v1::{CatalogInfo, CreateCatalogRequest};
use delta_sharing_common::models::credentials::v1::{
    CreateCredentialRequest, CredentialInfo, Purpose,
};
use delta_sharing_common::models::external_locations::v1::{
    CreateExternalLocationRequest, ExternalLocationInfo,
};
use delta_sharing_common::models::recipients::v1::{CreateRecipientRequest, RecipientInfo};
use delta_sharing_common::models::schemas::v1::{CreateSchemaRequest, SchemaInfo};
use delta_sharing_common::rest::client::UnityCatalogClient;
use futures::TryStreamExt;
use tauri::State;

use crate::error::Result;

#[tauri::command]
pub async fn list_catalogs(
    state: State<'_, UnityCatalogClient>,
    max_results: Option<i32>,
) -> Result<Vec<CatalogInfo>> {
    Ok(state.catalogs().list(max_results).try_collect().await?)
}

#[tauri::command]
pub async fn get_catalog(
    state: State<'_, UnityCatalogClient>,
    name: String,
) -> Result<CatalogInfo> {
    Ok(state.catalogs().get(name).await?)
}

#[tauri::command]
pub async fn create_catalog(
    state: State<'_, UnityCatalogClient>,
    request: CreateCatalogRequest,
) -> Result<CatalogInfo> {
    Ok(state.catalogs().create_catalog(&request).await?)
}

#[tauri::command]
pub async fn delete_catalog(
    state: State<'_, UnityCatalogClient>,
    name: String,
    force: Option<bool>,
) -> Result<()> {
    Ok(state.catalogs().delete(name, force).await?)
}

#[tauri::command]
pub async fn list_schemas(
    state: State<'_, UnityCatalogClient>,
    catalog: String,
    max_results: Option<i32>,
) -> Result<Vec<SchemaInfo>> {
    Ok(state
        .schemas()
        .list(catalog, max_results)
        .try_collect()
        .await?)
}

#[tauri::command]
pub async fn get_schema(
    state: State<'_, UnityCatalogClient>,
    catalog: String,
    schema: String,
) -> Result<SchemaInfo> {
    Ok(state.schemas().get(catalog, schema).await?)
}

#[tauri::command]
pub async fn create_schema(
    state: State<'_, UnityCatalogClient>,
    request: CreateSchemaRequest,
) -> Result<SchemaInfo> {
    Ok(state.schemas().create_schema(&request).await?)
}

#[tauri::command]
pub async fn delete_schema(
    state: State<'_, UnityCatalogClient>,
    catalog: String,
    name: String,
    force: Option<bool>,
) -> Result<()> {
    Ok(state.schemas().delete(catalog, name, force).await?)
}

#[tauri::command]
pub async fn list_credentials(
    state: State<'_, UnityCatalogClient>,
    purpose: Option<Purpose>,
    max_results: Option<i32>,
) -> Result<Vec<CredentialInfo>> {
    Ok(state
        .credentials()
        .list(purpose, max_results)
        .try_collect()
        .await?)
}

#[tauri::command]
pub async fn get_credential(
    state: State<'_, UnityCatalogClient>,
    name: String,
) -> Result<CredentialInfo> {
    Ok(state.credentials().get(name).await?)
}

#[tauri::command]
pub async fn create_credential(
    state: State<'_, UnityCatalogClient>,
    request: CreateCredentialRequest,
) -> Result<CredentialInfo> {
    Ok(state.credentials().create_credential(&request).await?)
}

#[tauri::command]
pub async fn delete_credential(state: State<'_, UnityCatalogClient>, name: String) -> Result<()> {
    Ok(state.credentials().delete(name).await?)
}

#[tauri::command]
pub async fn list_external_locations(
    state: State<'_, UnityCatalogClient>,
    max_results: Option<i32>,
) -> Result<Vec<ExternalLocationInfo>> {
    Ok(state
        .external_locations()
        .list(max_results)
        .try_collect()
        .await?)
}

#[tauri::command]
pub async fn get_external_location(
    state: State<'_, UnityCatalogClient>,
    name: String,
) -> Result<ExternalLocationInfo> {
    Ok(state.external_locations().get(name).await?)
}

#[tauri::command]
pub async fn create_external_location(
    state: State<'_, UnityCatalogClient>,
    request: CreateExternalLocationRequest,
) -> Result<ExternalLocationInfo> {
    Ok(state
        .external_locations()
        .create_external_location(&request)
        .await?)
}

#[tauri::command]
pub async fn delete_external_location(
    state: State<'_, UnityCatalogClient>,
    name: String,
    force: Option<bool>,
) -> Result<()> {
    Ok(state.external_locations().delete(name, force).await?)
}

#[tauri::command]
pub async fn list_recipients(
    state: State<'_, UnityCatalogClient>,
    max_results: Option<i32>,
) -> Result<Vec<RecipientInfo>> {
    Ok(state.recipients().list(max_results).try_collect().await?)
}

#[tauri::command]
pub async fn get_recipient(
    state: State<'_, UnityCatalogClient>,
    name: String,
) -> Result<RecipientInfo> {
    Ok(state.recipients().get(name).await?)
}

#[tauri::command]
pub async fn create_recipient(
    state: State<'_, UnityCatalogClient>,
    request: CreateRecipientRequest,
) -> Result<RecipientInfo> {
    Ok(state.recipients().create_recipient(&request).await?)
}

#[tauri::command]
pub async fn delete_recipient(state: State<'_, UnityCatalogClient>, name: String) -> Result<()> {
    Ok(state.recipients().delete(name).await?)
}
