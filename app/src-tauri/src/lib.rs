use cloud_client::CloudClient;
use delta_sharing_common::rest::client::UnityCatalogClient;
use tauri::{Builder, Manager};
use url::Url;

mod client;
mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = CloudClient::new_unauthenticated();
    let url = Url::parse("http://localhost:8080").unwrap();
    let unity_client = UnityCatalogClient::new(client, url);

    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            client::list_catalogs,
            client::get_catalog,
            client::create_catalog,
            client::delete_catalog,
            client::list_schemas,
            client::get_schema,
            client::create_schema,
            client::delete_schema,
            client::list_credentials,
            client::get_credential,
            client::create_credential,
            client::delete_credential,
            client::list_external_locations,
            client::get_external_location,
            client::create_external_location,
            client::delete_external_location,
            client::list_recipients,
            client::get_recipient,
            client::create_recipient,
            client::delete_recipient,
            client::list_shares,
            client::get_share,
            client::create_share,
            client::delete_share,
            client::list_table_summaries,
            client::list_tables,
            client::get_table,
            client::create_table,
            client::delete_table,
        ])
        .setup(|app| {
            app.manage(unity_client);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
