pub mod db;
pub mod store_comm;

use db::crud::{net_worth, transaction_amount_over_period, add_transaction};
use db::provider::{get_driver, initialize_tables, is_db_connected};
use tauri_plugin_os::platform;
use tauri_plugin_store::StoreExt;

#[tauri::command]
async fn connect_to_db(conn_string: Option<String>) -> &'static str {
    get_driver(conn_string).await;
    initialize_tables().await;

    if is_db_connected().await {
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
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_sql::Builder::new().build());
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            connect_to_db,
            tauri_platform,
            net_worth,
            transaction_amount_over_period,
            add_transaction
        ])
        .setup(|app| {
            let result = app.store("default.json");

            match result {
                Ok(store) => {
                    let is_onboarded: Option<tauri_plugin_store::JsonValue> =
                        store.get("is_onboarded");

                    if is_onboarded.is_none() {
                        store.set("is_onboarded", false);
                    }
                }
                Err(err) => {}
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
