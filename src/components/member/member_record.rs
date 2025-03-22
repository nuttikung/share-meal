use dioxus::{logger::tracing, prelude::*};
use dioxus_material_icons::MaterialIcon;

use crate::state::app_state::AppState;

#[derive(PartialEq, Props, Clone)]
pub struct MemberRecordProps {
    name: String,
    paid: bool,
}

#[component]
pub fn MemberRecord(props: MemberRecordProps) -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let member_name = String::clone(&props.name);

    // find the price for one person
    let bind_context = context.read();
    let current_member = bind_context.members.iter().find(|m| m.name == member_name);
    let x = 30;
    tracing::debug!("{:?}", current_member);

    // region :      --- Handle Payment Click
    let handle_payment_click = move |_| {
        context.write().members.iter_mut().for_each(|m| {
            if m.name == member_name {
                m.paid = !m.paid;
            }
        });
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
                    MaterialIcon { name: "check" }
                } else {
                    MaterialIcon { name: "clear" }
                }
            }
        }
    )
}
