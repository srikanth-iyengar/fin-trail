use super::side_app_bar::SideBar;
use leptos::prelude::*;
use leptos::*;
use leptos_router::components::Outlet;

#[component]
pub fn HomeLayout() -> impl IntoView {
    view! {
        <div class="home-layout">
            <SideBar />
            <div>
                <Outlet />
            </div>
        </div>
    }
}
