use delta_sharing_common::models::catalogs::v1::{CatalogInfo, CreateCatalogRequest};
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
