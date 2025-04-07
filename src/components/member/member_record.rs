use dioxus::prelude::*;

use crate::{helper::price::round_up_float_to_one_precision, state::app_state::AppState};

#[derive(PartialEq, Props, Clone)]
pub struct MemberRecordProps {
    name: String,
    paid: bool,
}

#[component]
pub fn MemberRecord(props: MemberRecordProps) -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let member_name = props.name.to_string();
    let price: f64 = context
        .read()
        .orders
        .iter()
        .filter(|o| o.has_member_in(&member_name))
        .map(|o| o.calculate_price_per_member())
        .sum();
    let round_up_price = round_up_float_to_one_precision(price);

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
                "{round_up_price}"
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
