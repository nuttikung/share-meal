use dioxus::prelude::*;

use crate::state::app_state::AppState;

#[component]
pub fn MemberList() -> Element {
    let context = use_context::<Signal<AppState>>();
    rsx! {
        div {
            class: "member-overview",
            id: "overview",

            div {
                class: "member-list",
                "ชื่อคน"
            }

            div {
                class: "price-payment",
                "จ่าย"
            }
        }

        for person in &context.read().members {
            div {
                class: "member-overview",
                id: "overview",

                div {
                    class: "member-list",
                    "{person}"
                }

                div {
                    class: "price-payment",
                    "จ่าย"
                }
            }
        }

    }
}
