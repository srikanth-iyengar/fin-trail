use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use super::provider::get_driver;
use super::provider::DbProvider;
use super::table::ACCOUNT_TB;
use super::table::TRANSACTION_TB;

pub async fn add_transaction() {}

pub async fn modify_transaction() {}

pub async fn delete_transaction() {}

pub async fn add_account() {}

pub async fn delete_account() {}

pub async fn add_recurrent_transaction() {}

pub async fn remove_recurrent_transaction() {}

pub async fn net_worth() -> Result<f64, String> {
    let arc = get_driver(None).await;
    let net_worth: f64;

    {
        let query = format!(
            "SELECT CAST(COALESCE((SELECT SUM(balance) FROM {}), 0) as double precision) as net_worth",
            ACCOUNT_TB
        );
        let mut driver = arc.lock().expect("failed to lock db driver");

        match &mut *driver {
            DbProvider::Sqlite(driver) => {
                let result: (f64,) = sqlx::query_as(&query)
                    .fetch_one(&mut driver.conn)
                    .await
                    .map_err(|e| e.to_string())?;
                net_worth = result.0;
            }
            DbProvider::Postgres(driver) => {
                let result: (f64,) = sqlx::query_as(&query)
                    .fetch_one(&driver.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                net_worth = result.0;
            }
            _ => {
                return Err("Unsupported database provider".to_string());
            }
        }
    }

    Ok(net_worth)
}

pub async fn transaction_amount_over_period(
    from_time: u64,
    to_time: u64,
    direction: bool,
) -> Result<f64, String> {
    let arc = get_driver(None).await;
    let income: f64;

    {
        let mut driver = arc.lock().expect("failed to lock db driver");

        let query = format!(
            r#"SELECT CAST(
            COALESCE(
        (SELECT SUM(amount) FROM {}
         WHERE timestamp BETWEEN $1 AND $2 AND direction = $3),
    0) as double precision) as income"#,
            TRANSACTION_TB
        );
        match &mut *driver {
            DbProvider::Sqlite(driver) => {
                let result: (f64,) = sqlx::query_as(&query)
                    .bind(from_time as i64)
                    .bind(to_time as i64)
                    .bind(direction)
                    .fetch_one(&mut driver.conn)
                    .await
                    .map_err(|e| e.to_string())?;
                income = result.0;
            }
            DbProvider::Postgres(driver) => {
                let result: (f64,) = sqlx::query_as(&query)
                    .bind(from_time as i64)
                    .bind(to_time as i64)
                    .bind(direction)
                    .fetch_one(&driver.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                income = result.0;
            }
            _ => return Err("Unsupported database provider".to_string()),
        }
    }

    Ok(income)
}
