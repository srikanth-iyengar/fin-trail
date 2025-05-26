use std::sync::{Arc, LazyLock, Mutex};

use crate::db::{postgres::PostgresDriver, sqlite::SqliteDriver};

pub enum DbProvider {
    Sqlite(SqliteDriver),
    Postgres(PostgresDriver),
    Unknown,
}

fn get_host_information() -> &'static str {
    tauri_plugin_os::platform()
}

pub static DRIVER: LazyLock<Arc<Mutex<DbProvider>>> =
    LazyLock::new(|| Arc::new(Mutex::new(DbProvider::Unknown)));

pub async fn get_driver(conn_string: Option<String>) -> Arc<Mutex<DbProvider>> {
    let platform = get_host_information();
    let arc = DRIVER.clone();

    {
        let mut locked = arc.lock().expect("Failed to lock DB driver mutex");
        if matches!(*locked, DbProvider::Unknown) {
            *locked = match platform {
                "android" => match SqliteDriver::connect(String::from("")).await {
                    Ok(driver) => DbProvider::Sqlite(driver),
                    Err(_) => DbProvider::Unknown,
                },
                "linux" => match PostgresDriver::connect(conn_string.unwrap()).await {
                    Ok(driver) => DbProvider::Postgres(driver),
                    Err(_) => DbProvider::Unknown,
                },
                _ => DbProvider::Unknown,
            }
        }
    }

    arc
}

pub fn is_db_connected() -> bool {
    let arc = DRIVER.clone();

    let locked = arc.lock().expect("Failed to lock DB driver mutex");
    !matches!(*locked, DbProvider::Unknown)
}
