use leptos::prelude::*;

#[derive(Clone)]
struct TabData {
    name: String,
    key: String,
    value: u32,
}

#[component]
pub fn Transaction() -> impl IntoView {
    let (active_tab, set_active_tab) = signal("all");
    let (tabs, _) = signal(vec![
        (
            "all",
            TabData {
                name: "All".to_string(),
                key: "all".to_string(),
                value: 10,
            },
        ),
        (
            "income",
            TabData {
                name: "Income".to_string(),
                key: "income".to_string(),
                value: 99,
            },
        ),
        (
            "expense",
            TabData {
                name: "Expense".to_string(),
                key: "expense".to_string(),
                value: 108,
            },
        ),
    ]);

    let change_active_tab = move |tab_key| {
        set_active_tab.set(tab_key);
    };

    view! {
        <div class="tx">
            <h1>Transactions</h1>
            <div class="tabs">
                <For
                    each=move || tabs.get()
                    key=|(_, tab)| tab.name.clone()
                    children=move |(id, tab)| view!{
                        <div on:click=move |_| change_active_tab(id)
                            class={move || format!("tab {}", if tab.key == active_tab.get() { "active" } else { "" })}>
                            <p class="title">{tab.name}</p>
                            <div class="tag">{tab.value}</div>
                        </div>
                    }
                />
            </div>
        </div>
    }
}
