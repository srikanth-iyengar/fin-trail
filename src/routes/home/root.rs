use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::transaction::{fetch_expense, fetch_income, fetch_networth};

#[derive(Serialize, Deserialize)]
pub struct TxAmountQuery {
    #[serde(rename = "fromTime")]
    pub from_time: u64,
    #[serde(rename = "toTime")]
    pub to_time: u64,
    #[serde(rename = "direction")]
    pub direction: bool,
}

#[component]
pub fn RootHome() -> impl IntoView {
    let now = Local::now().timestamp_millis();
    let ts_start_of_month = {
        let now = Local::now();
        let first_day_of_month = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(now.year(), now.month(), 1_u32).unwrap(),
            NaiveTime::from_hms_opt(0_u32, 0_u32, 0_u32).unwrap(),
        );
        first_day_of_month.and_utc().timestamp_millis()
    };

    let income_ref = LocalResource::new(move || fetch_income(ts_start_of_month as u64, now as u64));
    let expense_ref =
        LocalResource::new(move || fetch_expense(ts_start_of_month as u64, now as u64));
    let net_worth_ref = LocalResource::new(fetch_networth);

    let income = move || format!("{:.2} INR", income_ref.get().unwrap_or(0_f64));

    let expense = move || format!("{:.2} INR", expense_ref.get().unwrap_or(0_f64));
    let net_worth = move || format!("{:.2} INR", net_worth_ref.get().unwrap_or(0_f64));

    view! {
        <div class="home">
            <div class="item-mobile net-worth">
                <div class="item-header">
                    <img class="icon" src="/public/icons/piggy-bank.svg" />
                    <h4>Net worth</h4>
                </div>
                <h1>{net_worth}</h1>
            </div>
            <div class="item-mobile income">
                <div class="item-header">
                    <img class="icon" src="/public/icons/download-simple.svg" />
                    <h4>Income</h4>
                </div>
                <h2>{income}</h2>
            </div>
            <div class="item-mobile expense">
                <div class="item-header">
                    <img class="icon" src="/public/icons/upload-simple.svg" />
                    <h4>Expense</h4>
                </div>
                <h2>{expense}</h2>
            </div>
            <div class="item-desktop net-worth">
                <img class="icon" src="/public/icons/piggy-bank.svg" />
                <div class="item-header">
                    <h4>Net worth</h4>
                    <h2>{net_worth}</h2>
                </div>
            </div>
            <div class="item-desktop expense">
                <img class="icon" src="/public/icons/upload-simple.svg" />
                <div class="item-header">
                    <h4>Expense</h4>
                    <h2>{expense}</h2>
                </div>
            </div>
            <div class="item-desktop income">
                <img class="icon" src="/public/icons/download-simple.svg" />
                <div class="item-header">
                    <h4>Income</h4>
                    <h2>{income}</h2>
                </div>
            </div>
        </div>
    }
}
