use crate::components::home_layout::HomeLayout;
use crate::routes::home::account::Account;
use crate::routes::home::reccuring_transaction::RecurringTransaction;
use crate::routes::home::root;
use crate::routes::home::transaction::Transaction;
use crate::routes::init::postgres;
use crate::routes::init::sqlite3;
use crate::routes::splash;
use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
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
                <ParentRoute path=path!("/home") view=HomeLayout>
                    <Route path=path!("/root") view=RootHome />
                    <Route path=path!("/tx") view=Transaction />
                    <Route path=path!("/accounts") view=Account />
                    <Route path=path!("/recurring-transactions") view=RecurringTransaction />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
