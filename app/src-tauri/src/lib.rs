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
            client::create_catalog
        ])
        .setup(|app| {
            app.manage(unity_client);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
