use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;

use crate::{
    helper::price::round_up_float_to_one_precision,
    state::{app_state::AppState, member::Members},
};

#[derive(PartialEq, Props, Clone)]
pub struct MemberRecordProps {
    name: String,
    paid: bool,
}

#[component]
pub fn MemberRecord(props: MemberRecordProps) -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let member_name = props.name.to_string();
    let member_for_delete = member_name.clone();
    let price: f64 = context
        .read()
        .orders
        .iter()
        .filter(|o| o.has_member_in(&member_name))
        .map(|o| o.calculate_price_per_member())
        .sum();
    let round_up_price = round_up_float_to_one_precision(price);

    // region :      --- Handle Remove Member
    let handle_delete_member = move |_| {
        let current_members = context.read().members.clone();
        let update_members = exclude_member(&current_members, &member_for_delete);
        let current_orders = context.read().orders.clone();
        let update_orders = current_orders
            .into_iter()
            .map(|mut o| {
                o.remove_member(&member_for_delete);
                return o;
            })
            .collect();
        context.write().members = update_members;
        context.write().orders = update_orders;
    };
    // end region :  --- Handle Remove Member

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
        li {
            div {
                class: "group flex w-full items-center justify-between space-x-3
                        rounded-xl border border-gray-300 p-2 text-left shadow-xs
                        hover:bg-gray-50 focus:ring-2 focus:ring-offset-2 focus:outline-hidden
                        transform transition duration-500 hover:scale-105",
                span {
                    class: "flex min-w-0 flex-1 items-center space-x-3",
                    span {
                        class: "block min-w-0 flex-1",
                        span {
                            class: "block truncate text-sm font-medium text-gray-900",
                            "{props.name}"
                        }
                        span {
                            class: "block truncate text-sm font-medium text-gray-500",
                            "{round_up_price} บาท"
                        }

                        div {
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
                }

                span {
                    class: "inline-flex size-10 shrink-0 items-center justify-center",
                    button {
                        class: "text-red-400 cursor-pointer",
                        // class: "flex justify-center items-center p-1.5 text-gray",
                        r#type: "button",
                        onclick: handle_delete_member,
                        MaterialIcon {
                            name: "delete",
                            size: 20,
                        }
                    }
                }
            }
        }
    )
}

// TODO: move to helper
fn exclude_member(members: &Members, name: &str) -> Members {
    let tmp = members.clone();
    tmp.iter()
        .filter(|&m| m.name != name)
        .map(|m| m.clone())
        .collect()
}
