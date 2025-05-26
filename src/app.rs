use crate::routes::init::postgres;
use crate::routes::splash;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use postgres::PostgresInit;
use splash::Splash;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=Splash>
                <Route
                    path=path!("/splash")
                    view=Splash
                />
                <Route
                    path=path!("/init/postgres")
                    view=PostgresInit
                />
            </Routes>
        </Router>
    }
}
