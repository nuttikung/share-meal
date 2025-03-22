use dioxus::prelude::*;

use crate::{components::member::member_record::MemberRecord, state::app_state::AppState};

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
                "ราคา"
            }

            div {
                class: "price-payment",
                "จ่าย"
            }
        }

        for person in &context.read().members {
            MemberRecord { name: "{person.name}", paid: person.paid }
        }

    }
}
