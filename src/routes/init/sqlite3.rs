use crate::{
    components::button::Button,
    store::load,
    utils::{invokeNoArgs, invokeNoArgsNoPromise},
};
use js_sys::Math::log;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DbConnParams {
    #[serde(rename = "connString")]
    conn_string: String,
}

#[component]
pub fn SqliteInit() -> impl IntoView {
    let navigate = use_navigate();
    let store = LocalResource::new(move || load("default.json"));
    let store_value = move || store.get();

    let proceed_action = Action::new(move |_: &String| {
        let nav = navigate.clone();
        async move {
            let store = store.get().unwrap();
            store.set("is_onboarded", true.into());
            invokeNoArgsNoPromise("connect_to_db");
            nav("/home/root", NavigateOptions::default());
        }
    });

    let route_to_home = move || {
        proceed_action.dispatch_local("proceed".to_string());
    };

    view! {
        <div class="postgres" style="height: 100vh;">
            <h1>You are using android device</h1>
            <img src="/public/external/sqlite.svg" class="logo" alt="Sqlite logo" />
            <p>Will be using sqlite for storage and can opt-in to seamlessly sync with a linux based host</p>
            <div style="margin-top: 1rem;">
                <Button disabled={false} on_click=route_to_home color="red-btn">{
                           move || if store_value().is_none() { "Disabled" } else { "Next" }
    }</Button>
            </div>
        </div>
    }
}
