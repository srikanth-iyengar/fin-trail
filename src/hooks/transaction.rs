use crate::{
    routes::home::root::TxAmountQuery,
    utils::{invoke, invokeNoArgs},
};

pub async fn fetch_networth() -> f64 {
    invokeNoArgs("net_worth").await.as_f64().unwrap_or(0_f64)
}

pub async fn fetch_expense(start: u64, end: u64) -> f64 {
    invoke(
        "transaction_amount_over_period",
        wasm_bindgen::JsValue::from_serde(&TxAmountQuery {
            from_time: start,
            to_time: end,
            direction: false,
        })
        .unwrap(),
    )
    .await
    .as_f64()
    .unwrap_or(0_f64)
}

pub async fn fetch_income(start: u64, end: u64) -> f64 {
    invoke(
        "transaction_amount_over_period",
        wasm_bindgen::JsValue::from_serde(&TxAmountQuery {
            from_time: start,
            to_time: end,
            direction: true,
        })
        .unwrap(),
    )
    .await
    .as_f64()
    .unwrap_or(0_f64)
}
