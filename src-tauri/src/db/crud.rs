use super::provider::get_driver;
use super::provider::DbProvider;
use super::table::ACCOUNT_TB;
use super::table::TRANSACTION_TB;
use crate::match_driver;

#[tauri::command]
pub async fn net_worth() -> Result<f64, String> {
    let arc = get_driver(None).await;
    let mut net_worth: f64 = 0_f64;
    let query = format!(
        "SELECT CAST(COALESCE((SELECT SUM(balance) FROM {}), 0) as double precision) as net_worth",
        ACCOUNT_TB
    );
    let mut driver = arc.lock().await;

    match_driver!(&mut *driver, db_driver -> {
        let result: (f64,) = sqlx::query_as(&query)
            .fetch_one(&db_driver.pool)
            .await
            .map_err(|e| e.to_string())?;
        net_worth = result.0;
    });

    Ok(net_worth)
}

#[tauri::command]
pub async fn transaction_amount_over_period(
    from_time: u64,
    to_time: u64,
    direction: bool,
) -> Result<f64, String> {
    let arc = get_driver(None).await;
    let mut income: f64 = 0_f64;
    let mut driver = arc.lock().await;
    let query = format!(
        r#"SELECT CAST(
            COALESCE(
        (SELECT SUM(amount) FROM {}
         WHERE ts BETWEEN $1 AND $2 AND direction = $3),
    0) as double precision) as income"#,
        TRANSACTION_TB
    );
    match_driver!(&mut *driver, db_driver -> {
            let result: (f64,) = sqlx::query_as(&query)
                .bind(from_time as i64)
                .bind(to_time as i64)
                .bind(direction)
                .fetch_one(&db_driver.pool)
                .await
                .map_err(|e| e.to_string())?;
            income = result.0;
    });
    Ok(income)
}
