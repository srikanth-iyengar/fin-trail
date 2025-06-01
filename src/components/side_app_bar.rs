use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[derive(Clone)]
struct Tab {
    name: String,
    href: String,
    icon: String,
}

#[component]
pub fn SideBar() -> impl IntoView {
    let location = use_location();
    let path = move || location.pathname.get();

    let tabs = vec![
        Tab {
            name: "Home".to_string(),
            href: "/home/root".to_string(),
            icon: "/public/icons/home.svg".to_string(),
        },
        Tab {
            name: "Transaction".to_string(),
            href: "/home/tx".to_string(),
            icon: "/public/icons/wallet.svg".to_string(),
        },
        Tab {
            name: "Account".to_string(),
            href: "/home/accounts".to_string(),
            icon: "/public/icons/credit-card.svg".to_string(),
        },
        Tab {
            name: "Rec. Transaction".to_string(),
            href: "/home/recurring-transactions".to_string(),
            icon: "/public/icons/arrows-clockwise.svg".to_string(),
        },
    ];

    view! {
        <div class="side-bar">
            <div class="header">
                <img class="logo" src="/public/logo.svg" alt="Fin Trail" />
                <h2>Fin Trail</h2>
            </div>
            <For
                each=move || tabs.clone()
                key=|tab| tab.href.clone()
                children=move |tab: Tab| {
                    view!{
                        <div class=move || format!("item {}", if path() == tab.href { "active" } else { "" })>
                            <img class="icon" src={tab.icon} />
                            <A href={tab.href.clone()}>{tab.name.clone()}</A>
                        </div>
                    }
                }
            />
        </div>
    }
}
