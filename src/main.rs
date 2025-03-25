use dioxus::prelude::*;
use dioxus_material_icons::MaterialIconStylesheet;
use shared_meal::{
    components::{
        member::{
            member_clear_button::MemberClearButton, member_input::MemberInput,
            member_list::MemberList,
        },
        order::{order_input::OrderInput, order_list::OrderList},
        stats::stats_overview::StatsOverview,
        tab::tab_view_switcher::TabViewSwitcher,
    },
    state::app_state::AppState,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

// region :      --- Main Function
fn main() {
    dioxus::launch(App);
}
// end region :  --- Main Function

#[component]
fn App() -> Element {
    use_context_provider(|| {
        Signal::new(AppState {
            view: String::from("orders"),
            orders: vec![],
            members: vec![],
        })
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        MaterialIconStylesheet {}
        Overview {}
    }
}

#[component]
pub fn Overview() -> Element {
    let context = use_context::<Signal<AppState>>();
    let current_view = context().view;

    rsx! {
        main {
            class: "bg-neutral-50",
            div {
                class: "mx-auto max-w-5xl",
                StatsOverview {}
                TabViewSwitcher {}

                if current_view == "orders" {
                    OrderList {}
                    OrderInput {}
                } else {
                    MemberList {}
                    MemberInput {}
                    MemberClearButton {}
                }
            }
        }
    }
}
