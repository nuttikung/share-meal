use dioxus::prelude::*;

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
        tr {
            td {
                class: "py-4 pr-3 pl-4 text-sm font-medium whitespace-nowrap text-gray-900 sm:pl-0",
                "{props.name}"
            }
            td {
                class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                "{x}"
            }
            td {
                class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                if props.paid {
                    span {
                        class: "inline-flex items-center rounded-md bg-green-100 px-2 py-1 text-xs font-medium text-green-700 cursor-pointer",
                        onclick: handle_payment_click,
                        "จ่ายแล้ว"
                    }
                } else {
                    span {
                        class: "inline-flex items-center rounded-md bg-red-100 px-2 py-1 text-xs font-medium text-red-700 cursor-pointer",
                        onclick: handle_payment_click,
                        "ยังไม่จ่าย"
                    }
                }
            }
        }
    )
}
