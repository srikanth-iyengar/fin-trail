use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

use crate::db::{postgres::PostgresDriver, sqlite::SqliteDriver};

use super::{
    driver::Driver,
    table::{ACCOUNT_TB, ACC_TABLE, REC_TX_TABLE, REC_TX_TB, TRANSACTION_TB, TX_TABLE},
};

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
        let mut locked = arc.lock().await;
        if matches!(*locked, DbProvider::Unknown) {
            *locked = match platform {
                "android" => match SqliteDriver::connect(String::from("")).await {
                    Ok(driver) => DbProvider::Sqlite(driver),
                    Err(err) => {
                        eprintln!("{:?}", err);
                        DbProvider::Unknown
                    }
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

pub async fn initialize_tables() {
    let arc_driver = get_driver(None).await;
    {
        let mut locked_driver = arc_driver.lock().await;
        match &mut *locked_driver {
            DbProvider::Sqlite(driver) => {
                driver
                    .create_table(TRANSACTION_TB.to_string(), TX_TABLE.to_vec())
                    .await;
                driver
                    .create_table(ACCOUNT_TB.to_string(), ACC_TABLE.to_vec())
                    .await;
                driver
                    .create_table(REC_TX_TB.to_string(), REC_TX_TABLE.to_vec())
                    .await;
            }
            DbProvider::Postgres(driver) => {
                driver
                    .create_table(TRANSACTION_TB.to_string(), TX_TABLE.to_vec())
                    .await;
                driver
                    .create_table(ACCOUNT_TB.to_string(), ACC_TABLE.to_vec())
                    .await;
                driver
                    .create_table(REC_TX_TB.to_string(), REC_TX_TABLE.to_vec())
                    .await;
            }
            _ => {}
        }
    }
}

pub async fn is_db_connected() -> bool {
    let arc = DRIVER.clone();

    let locked = arc.lock().await;
    !matches!(*locked, DbProvider::Unknown)
}
