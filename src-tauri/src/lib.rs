pub mod db;

use db::provider::{get_driver, is_db_connected};
use tauri_plugin_os::platform;
use tauri_plugin_store::StoreExt;

#[tauri::command]
fn connect_to_db(conn_string: String) -> &'static str {
    tauri::async_runtime::block_on(get_driver(Some(conn_string)));

    if is_db_connected() {
        return "connected";
    }
    "disconnected"
}

#[tauri::command]
fn tauri_platform() -> &'static str {
    platform()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![connect_to_db, tauri_platform])
        .setup(|app| {
            let store = app.store("settings.json")?;
            store.close_resource();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
