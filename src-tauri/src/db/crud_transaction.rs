use super::provider::get_driver;
use super::provider::DbProvider;
use crate::db::driver::build_where_clause;
use crate::db::provider::SqlxType;
use crate::match_driver;
use common::table::{Condition, Value};
use common::table::{Transaction, TRANSACTION_TB};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[tauri::command]
pub async fn add_transaction(
    amount: f64,
    ts: Option<u128>,
    direction: bool,
    is_synced: bool,
    tags: String,
) -> Result<(), String> {
    let arc = get_driver(None).await;
    let tx_id = Uuid::new_v4();
    let ts = ts.unwrap_or(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    );

    let query = format!(
        r#"INSERT INTO {} (
                    tx_id, ts, amount, direction, is_synced, tags
                ) VALUES ($1, $2, $3, $4, $5, $6)"#,
        TRANSACTION_TB
    );
    let mut driver = arc.lock().await;

    match_driver!(&mut *driver, db_driver -> {
            let result = sqlx::query(&query)
                .bind(tx_id.to_string())
                .bind(ts as i64)
                .bind(amount)
                .bind(direction)
                .bind(is_synced)
                .bind(tags)
                .execute(&db_driver.pool)
                .await
                .map_err(|e| e.to_string())?;
            if result.rows_affected() == 1 {
                return Ok(());
            }
    });
    Err("Could not insert transaction".to_string())
}

#[tauri::command]
pub async fn fetch_transaction(filter: Vec<Condition>) -> Vec<Transaction> {
    let driver = get_driver(None).await;
    let (where_clause, values) = build_where_clause(filter);
    let query_tmpl = format!("SELECT * FROM {TRANSACTION_TB} {where_clause}");
    let mut result: Vec<Transaction> = vec![];

    let mut query: sqlx::query::QueryAs<'_, SqlxType, Transaction, _> = sqlx::query_as(&query_tmpl);
    for val in values.iter() {
        match val {
            Value::StringVal(value) => {
                query = query.bind(value);
            }
            Value::Number(value) => {
                query = query.bind(value);
            }
            Value::Boolean(value) => {
                query = query.bind(value);
            }
        }
    }

    let mut locked_driver = driver.lock().await;
    match_driver!(&mut *locked_driver, driver -> {
        let transactions: Result<Vec<Transaction>, sqlx::Error> = query.fetch_all(&driver.pool).await;
        result = transactions.unwrap_or_default();
    });

    result
}

#[tauri::command]
pub async fn fetch_transaction_count() -> Result<(i64, i64, i64), String> {
    let query_tmpl = format!(r#"
    SELECT
        COUNT(*) AS total,
        SUM(CASE WHEN direction = true THEN 1 ELSE 0 END) AS inbound,
        SUM(CASE WHEN direction = false THEN 1 ELSE 0 END) AS outbound
    FROM {TRANSACTION_TB};
    "#);
    let mut result = (0_i64, 0_i64, 0_i64);

    let driver = get_driver(None).await;

    let mut locked_driver = driver.lock().await;

    match_driver!(&mut *locked_driver, driver -> {
        let count: (i64, i64, i64) = sqlx::query_as(&query_tmpl)
            .fetch_one(&driver.pool)
            .await
            .map_err(|e| e.to_string())?;
        result = count;
    });


    Ok(result)
}
