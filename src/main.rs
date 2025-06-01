mod app;
mod components;
mod routes;
mod store;
mod utils;

use app::*;

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
