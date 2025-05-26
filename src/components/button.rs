use leptos::prelude::*;
use leptos::*;

#[component]
pub fn Button<F>(on_click: F, color: &'static str, children: Children) -> impl IntoView
where
    F: Fn() + 'static,
{
    let class = format!("btn {}", color);

    view! {
        <button class={class} on:click=move |_| on_click()>
            {children()}
        </button>
    }
}
