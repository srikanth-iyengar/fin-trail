use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::utils::invokeNoArgs;

#[component]
pub fn Splash() -> impl IntoView {
    let navigate = use_navigate();
    let platform_data = LocalResource::new(move || invokeNoArgs("tauri_platform"));
    let result = move || {
        platform_data
            .get()
            .map(|value| value.as_string().unwrap_or("unknown".into()))
    };

    Effect::new(
        move || match result().unwrap_or("unknown".into()).as_str() {
            "linux" => {
                navigate("/init/postgres", leptos_router::NavigateOptions::default());
            }
            "android" => {
                navigate("/init/sqlite", leptos_router::NavigateOptions::default());
            }
            _ => {}
        },
    );

    view! {
       <div class="splash">
            <img src="/public/logo.svg" class="logo" />
            <h1 class="app-name">Fin Trail</h1>
            <p class="short-description">A open source finance manager written in rust, powered by tauri</p>
        </div>
    }
}
