use super::crud::{net_worth, transaction_amount_over_period};

#[tauri::command]
pub fn net_worth_wrapper() -> Result<f64, String> {
    let net_worth_future = net_worth();
    tauri::async_runtime::block_on(net_worth_future)
}

#[tauri::command]
pub fn tx_amount_over_period(from_time: u64, to_time: u64, direction: bool) -> Result<f64, String> {
    let amount_future = transaction_amount_over_period(from_time, to_time, direction);
    let result = tauri::async_runtime::block_on(amount_future);

    result
}
