use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

use crate::{
    db::postgres::PostgresDriver,
    match_driver,
};

use super::driver::Driver;
use common::table::{ACCOUNT_TB, ACC_TABLE, REC_TX_TABLE, REC_TX_TB, TRANSACTION_TB, TX_TABLE};

pub enum DbProvider {
    #[cfg(target_os = "android")]
    Sqlite(SqliteDriver),
    #[cfg(target_os = "linux")]
    Postgres(PostgresDriver),
    Unknown,
}

#[cfg(target_os = "android")]
pub type SqlxType = sqlx::Sqlite;
#[cfg(target_os = "linux")]
pub type SqlxType = sqlx::Postgres;

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
                #[cfg(target_os = "android")]
                "android" => match SqliteDriver::connect(String::from("")).await {
                    Ok(driver) => DbProvider::Sqlite(driver),
                    Err(err) => {
                        eprintln!("{:?}", err);
                        DbProvider::Unknown
                    }
                },
                #[cfg(target_os = "linux")]
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
        match_driver!(&mut *locked_driver, db_driver -> {
            db_driver
                .create_table(TRANSACTION_TB.to_string(), TX_TABLE.to_vec())
                .await;
            db_driver
               .create_table(ACCOUNT_TB.to_string(), ACC_TABLE.to_vec())
               .await;
            db_driver
               .create_table(REC_TX_TB.to_string(), REC_TX_TABLE.to_vec())
               .await;
        });
    }
}

pub async fn is_db_connected() -> bool {
    let arc = DRIVER.clone();

    let locked = arc.lock().await;
    !matches!(*locked, DbProvider::Unknown)
}
