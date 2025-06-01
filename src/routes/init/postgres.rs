use crate::utils::invoke;
use crate::{components::button::Button, store::load};
use leptos::prelude::*;
use leptos::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize)]
struct DbConnParams {
    #[serde(rename = "connString")]
    conn_string: String,
}

#[component]
pub fn PostgresInit() -> impl IntoView {
    let (connection_string, set_connection_string) = signal("".to_string());
    let navigate = use_navigate();
    let store = LocalResource::new(|| load("default.json"));

    let proceed_action = Action::new(move |_: &String| {
        let nav = navigate.clone();
        async move {
            let store = store.get().unwrap().take();
            store.set("is_onboarded", true.into());
            store.set("pg_url", connection_string.get().into());
            nav("/home/root", NavigateOptions::default());
        }
    });

    let connect_db = move || {
        let params = DbConnParams {
            conn_string: connection_string.get(),
        };

        spawn_local(async move {
            if let Some(status) = invoke(
                "connect_to_db",
                wasm_bindgen::JsValue::from_serde(&params).unwrap(),
            )
            .await
            .as_string()
            {
                proceed_action.dispatch("proceed".to_string());
            }
        });
    };

    view! {
        <div class="postgres" style="height: 100vh;">
            <h1>Enter Postgres Connection String</h1>
            <img src="/public/external/pg.svg" class="logo" alt="Postgres Logo" />
            <input
                type="text"
                placeholder="postgres://user:password@host/db"
                prop:value=connection_string
                on:input=move |ev| set_connection_string.set(event_target_value(&ev))
            />
            <div style="margin-top: 1rem;">
                <Button disabled={false} on_click=connect_db color="red-btn">Next</Button>
            </div>
        </div>
    }
}
