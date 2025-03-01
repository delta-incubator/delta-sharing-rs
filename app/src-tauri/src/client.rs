use delta_sharing_common::models::catalogs::v1::CatalogInfo;
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
