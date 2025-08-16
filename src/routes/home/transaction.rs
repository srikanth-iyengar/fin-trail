use common::table::{Condition, Operator};
use leptos::prelude::*;
use leptos_struct_table::*;

use crate::hooks::transaction::{fetch_transaction, fetch_transaction_counts};

#[derive(Clone)]
struct TabData {
    name: String,
    key: String,
    value: u32,
    filter: Vec<Condition>,
}

#[component]
pub fn Transaction() -> impl IntoView {
    let (active_tab, set_active_tab) = signal("all");
    let (filter, set_filter) = signal::<Vec<Condition>>(vec![]);

    let (tabs, _) = signal(vec![
        (
            0,
            "all",
            TabData {
                name: "All".to_string(),
                key: "all".to_string(),
                value: 10,
                filter: vec![],
            },
        ),
        (
            1,
            "income",
            TabData {
                name: "Income".to_string(),
                key: "income".to_string(),
                value: 99,
                filter: vec![Condition {
                    field: "direction".to_string(),
                    operator: Operator::Eq(common::table::Value::Boolean(true)),
                }],
            },
        ),
        (
            2,
            "expense",
            TabData {
                name: "Expense".to_string(),
                key: "expense".to_string(),
                value: 108,
                filter: vec![Condition {
                    field: "direction".to_string(),
                    operator: Operator::Eq(common::table::Value::Boolean(false)),
                }],
            },
        ),
    ]);

    Effect::new(move || {
        let active_tab = active_tab.get();
        match active_tab {
            "all" => {
                set_filter.set(tabs.get()[0].2.filter.clone());
            }
            "income" => {
                set_filter.set(tabs.get()[1].2.filter.clone());
            }
            "expense" => {
                set_filter.set(tabs.get()[2].2.filter.clone());
            }
            _ => set_filter.set(vec![]),
        }
    });

    let transaction_ref = LocalResource::new(move || fetch_transaction(filter.get()));
    let rows = move || transaction_ref.get().unwrap_or(vec![]);

    let transaction_cnt = LocalResource::new(fetch_transaction_counts);

    let change_active_tab = move |tab_key| {
        set_active_tab.set(tab_key);
    };

    view! {
        <div class="tx">
            <h1>Transactions</h1>
            <div class="tabs">
                <For
                    each=move || tabs.get()
                    key=|(_, _, tab)| tab.name.clone()
                    children=move |(idx, id, tab)| view!{
                        <div on:click=move |_| change_active_tab(id)
                            class={move || format!("tab {}", if tab.key == active_tab.get() { "active" } else { "" })}>
                            <p class="title">{tab.name}</p>
                            <div class="tag">{move || transaction_cnt.get().unwrap_or(vec![0, 0, 0])[idx as usize]}</div>
                        </div>
                    }
                />
            </div>
            <table>
                <tr>
                    <th>Date</th>
                    <th>Amount</th>
                    <th>Credit/Debit</th>
                    <th>Synced?</th>
                    <th>Tags</th>
                    <th>Account</th>
                </tr>
                <For
                    
                />
            </table>
        </div>
    }
}
