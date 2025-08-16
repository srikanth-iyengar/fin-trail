use common::table::Condition;
use serde::{Deserialize, Serialize};
use crate::dto::transaction::Transaction;

use crate::{
    routes::home::root::TxAmountQuery,
    utils::{invoke, invokeNoArgs, log},
};

#[derive(Serialize, Deserialize)]
struct FilterDto {
    filter: Vec<Condition>,
}

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

pub async fn fetch_transaction(filter: Vec<Condition>) -> Vec<Transaction> {
    let request = FilterDto {
        filter: filter.clone(),
    };
    let result = invoke(
        "fetch_transaction",
        wasm_bindgen::JsValue::from_serde(&request).unwrap(),
    )
    .await;

    result.into_serde().unwrap()
}

pub async fn fetch_transaction_counts() -> Vec<i64> {
    let result = invokeNoArgs("fetch_transaction_count").await;

    result.into_serde().unwrap()
}
