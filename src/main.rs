use dioxus::prelude::*;
use shared_meal::{
    components::{
        member::{
            member_clear_button::MemberClearButton, member_input::MemberInput,
            member_list::MemberList,
        },
        order::{order_input::OrderInput, order_list::OrderList},
    },
    state::app_state::AppState,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Overview {}
    }
}

#[component]
pub fn Overview() -> Element {
    let mut context = use_context::<Signal<AppState>>();

    let member_count = context.read().members.len();

    // region :      --- Handle Order Click
    let handleOrderClick = move |_| {
        context.write().view = String::from("orders");
    };

    let handleMemberClick = move |_| {
        context.write().view = String::from("members");
    };
    // end region :  --- Handle Order Click

    rsx! {
        main {
            div {
                id: "overview",
                div {
                    class: "overvie-item",
                    "จำนวนคน"
                    div {
                        id: "overvie-item-total-person",
                        class: "overvie-item-total",
                        "{member_count}"
                    }
                }

                div {
                    class: "overvie-item",
                    "ราคารวม"
                    div {
                        id: "overvie-item-total-price",
                        class: "overvie-item-total",
                        "0"
                    }
                }
            }


            div { class: "radio-inputs",
                label { class: "radio",
                    input { r#type: "radio", name: "radio", checked: context().view == "orders", onclick: handleOrderClick}
                    span { class: "name", "รายการ" }
                }
                label { class: "radio",
                    input { r#type: "radio", name: "radio", checked: context().view == "members", onclick: handleMemberClick}
                    span { class: "name", "คนจ่าย" }
                }
            }

            if context().view == "orders" {
                OrderList {}
                div {
                    OrderInput {}
                }
            } else {
                MemberList {}
                div {
                    MemberInput {}
                    MemberClearButton {}
                }
            }

        }
    }
}
