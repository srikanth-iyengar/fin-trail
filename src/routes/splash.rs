use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};

use crate::{
    store::load,
    utils::{invoke, invokeNoArgs},
};

#[derive(Serialize, Deserialize)]
struct DbConnParams {
    #[serde(rename = "connString")]
    conn_string: String,
}

#[component]
pub fn Splash() -> impl IntoView {
    let navigate = use_navigate();

    spawn_local(async move {
        let store = load("default.json").await;
        let platform = invokeNoArgs("tauri_platform")
            .await
            .as_string()
            .unwrap_or_else(|| "unknown".into());
        let is_onboarded = store.get("is_onboarded").await.as_bool().unwrap_or(false);
        match platform.as_str() {
            "linux" => {
                let pg_url = store
                    .get("pg_url")
                    .await
                    .as_string()
                    .unwrap_or("".to_string());
                let params = DbConnParams {
                    conn_string: pg_url.clone(),
                };
                invoke(
                    "connect_to_db",
                    wasm_bindgen::JsValue::from_serde(&params).unwrap(),
                )
                .await;
            }
            "android" => {
                invokeNoArgs("connect_to_db").await;
            }
            _ => {}
        }

        if is_onboarded {
            navigate("/home/root", leptos_router::NavigateOptions::default());
        } else {
            let route = match platform.as_str() {
                "linux" => Some("/init/postgres"),
                "android" => Some("/init/sqlite"),
                _ => None,
            };

            if let Some(path) = route {
                navigate(path, leptos_router::NavigateOptions::default());
            }
        }
    });

    view! {
        <div class="splash">
            <img src="/public/logo.svg" class="logo" />
            <h1 class="app-name">Fin Trail</h1>
            <p class="short-description">An open source finance manager written in Rust, powered by Tauri.</p>
        </div>
    }
}
