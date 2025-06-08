use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

use super::driver::Value;
use super::provider::DbProvider;
use super::{driver::Condition, provider::get_driver};
use crate::db::driver::build_where_clause;
use crate::db::provider::SqlxType;
use crate::db::table::{Transaction, TRANSACTION_TB};
use crate::match_driver;

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
        let transactions: Vec<Transaction> = query.fetch_all(&driver.pool).await.unwrap_or(vec![]);
        result = transactions;
    });

    result
}
