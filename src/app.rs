use crate::routes::home::root;
use crate::routes::init::postgres;
use crate::routes::init::sqlite3;
use crate::routes::splash;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use postgres::PostgresInit;
use root::RootHome;
use splash::Splash;
use sqlite3::SqliteInit;

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
                <Route
                    path=path!("/init/sqlite")
                    view=SqliteInit
                />
                <Route
                    path=path!("/home/root")
                    view=RootHome
                />
            </Routes>
        </Router>
    }
}
