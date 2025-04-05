use dioxus::{logger::tracing, prelude::*};

use crate::state::{app_state::AppState, member::Members};

#[derive(PartialEq, Props, Clone)]
pub struct MemberRecordProps {
    name: String,
    paid: bool,
}

#[component]
pub fn MemberRecord(props: MemberRecordProps) -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let member_name = String::clone(&props.name);

    let bind_context = context.read();
    let price: f64 = bind_context
        .orders
        .iter()
        .filter(|order| has_name_in(&order.members, &member_name))
        .map(|o| {
            if o.members.len() == 0 {
                return 0 as f64;
            } else {
                return o.price / o.members.len() as f64;
            }
        })
        .sum();
    let round_up_price = format!("{:.2}", price);

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

/// Utility function to check if name is in the Vec<Member> or Members
///
/// `true` when name is included,
///
/// `false` when name is not included
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let members = vec![{ name: "a", paid: false },{ name: "b", paid: false }]
/// assert_eq!(has_self_in(&members, "a"), true);
/// assert_eq!(has_self_in(&members, "c"), false);
/// ```
fn has_name_in(members: &Members, name: &str) -> bool {
    let mut found: bool = false;
    for n in members {
        if n.name == name {
            found = true;
            break;
        }
    }

    return found;
}
