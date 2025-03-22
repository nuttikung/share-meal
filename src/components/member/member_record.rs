use dioxus::{logger::tracing, prelude::*};
// use dioxus_free_icons::Icon;

use crate::state::app_state::AppState;

#[derive(PartialEq, Props, Clone)]
pub struct MemberRecordProps {
    name: String,
    paid: bool,
}

#[component]
pub fn MemberRecord(props: MemberRecordProps) -> Element {
    // find the price for one person
    let context = use_context::<Signal<AppState>>();
    let member_name = String::clone(&props.name);

    tracing::debug!("{} {}", props.name, props.paid);
    let x = 30;

    // region :      --- Handle Payment Click
    let handle_payment_click = move |_| {
        let temp_context = context.read();
        let current_member = temp_context
            .members
            .iter()
            .find(|m| m.name.to_string() == member_name.to_string());

        if current_member.is_none() {
            return
        }

        let m = current_member;

        m.unwrap().clone().paid = !m.unwrap().paid;

        tracing::debug!("{:?}", current_member);
    };
    // end region :  --- Handle Payment Click

    rsx!(
        div {
            class: "member-overview",
            id: "overview",

            div {
                class: "member-list",
                "{props.name}"
            }

            div {
                class: "price-payment",
                "{x}"
            }

            div {
                class: "price-payment",
                onclick: handle_payment_click,

                if props.paid {
                    "จ่ายแล้ว"
                } else {
                    "ยังไม่จ่าย"
                }
            }
        }
    )
}
