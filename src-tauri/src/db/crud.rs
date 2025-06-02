use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use uuid::Uuid;
use super::provider::get_driver;
use super::provider::DbProvider;
use super::table::ACCOUNT_TB;
use super::table::TRANSACTION_TB;

#[tauri::command]
pub async fn add_transaction(amount: f64, ts: Option<u128>, direction: bool, acc_id: String, is_synced: bool, tags: String) -> Result<(), String> {
    let arc = get_driver(None).await;
    let tx_id = Uuid::new_v4();
    let ts = ts.unwrap_or(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());

    {
        let query = format!(
            r#"INSERT INTO {} (
                    tx_id, acc_id, ts, amount, direction, is_synced, tags
                ) VALUES ($1, $2,$3, $4, $5, $6, $7)"#,
            TRANSACTION_TB
        );
        let mut driver = arc.lock().await;

        match &mut *driver {
            DbProvider::Sqlite(driver) => {
                let result = sqlx::query(&query)
                    .bind(tx_id.to_string())
                    .bind(acc_id)
                    .bind(ts as i64)
                    .bind(amount)
                    .bind(direction)
                    .bind(is_synced)
                    .bind(tags)
                    .execute(&driver.conn)
                    .await
                    .map_err(|e| e.to_string())?;
                if result.rows_affected() == 1 {
                    return Ok(());
                }
            }
            DbProvider::Postgres(driver) => {
                let result = sqlx::query(&query)
                    .bind(tx_id.to_string())
                    .bind(acc_id)
                    .bind(ts as i64)
                    .bind(amount)
                    .bind(direction)
                    .bind(is_synced)
                    .bind(tags)
                    .execute(&driver.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if result.rows_affected() == 1 {
                    return Ok(());
                }
            }
            _ => {
                return Err("Unsupported database provider".to_string());
            }
        }
        Err("Could not insert transaction".to_string())
    }
}

pub async fn modify_transaction() {}

pub async fn delete_transaction() {}

pub async fn add_account() {}

pub async fn delete_account() {}

pub async fn add_recurrent_transaction() {}

pub async fn remove_recurrent_transaction() {}

#[tauri::command]
pub async fn net_worth() -> Result<f64, String> {
    let arc = get_driver(None).await;
    let net_worth: f64;

    {
        let query = format!(
            "SELECT CAST(COALESCE((SELECT SUM(balance) FROM {}), 0) as double precision) as net_worth",
            ACCOUNT_TB
        );
        let mut driver = arc.lock().await;

        match &mut *driver {
            DbProvider::Sqlite(driver) => {
                let result: (f64,) = sqlx::query_as(&query)
                    .fetch_one(&driver.conn)
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

#[tauri::command]
pub async fn transaction_amount_over_period(
    from_time: u64,
    to_time: u64,
    direction: bool,
) -> Result<f64, String> {
    let arc = get_driver(None).await;
    let income: f64;

    {
        let mut driver = arc.lock().await;

        let query = format!(
            r#"SELECT CAST(
            COALESCE(
        (SELECT SUM(amount) FROM {}
         WHERE ts BETWEEN $1 AND $2 AND direction = $3),
    0) as double precision) as income"#,
            TRANSACTION_TB
        );
        match &mut *driver {
            DbProvider::Sqlite(driver) => {
                let result: (f64,) = sqlx::query_as(&query)
                    .bind(from_time as i64)
                    .bind(to_time as i64)
                    .bind(direction)
                    .fetch_one(&driver.conn)
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
