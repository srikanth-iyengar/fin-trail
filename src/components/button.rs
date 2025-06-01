use leptos::prelude::*;
use leptos::*;

#[component]
pub fn Button<F>(
    on_click: F,
    color: &'static str,
    disabled: bool,
    children: Children,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    let class = format!("btn {}", color);

    view! {
        <button disabled={disabled} class={class} on:click=move |_| on_click()>
            {children()}
        </button>
    }
}
