use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    components::order::{
        order_input::OrderInput, order_member_check_box::OrderMemberCheckBox,
        order_price_input::OrderPriceInput,
    },
    state::{
        app_state::AppState,
        member::{Member, Members},
        order::Order,
    },
};

#[component]
pub fn OrderInsert() -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let read_context = context.read();
    let mut title = use_signal(|| String::from(""));
    let mut price = use_signal(|| 0.0);
    let mut selected_member = use_signal(|| vec![] as Vec<Member>);

    // region :      --- Handle Price Input
    let handle_price_change = move |event: Event<FormData>| {
        let input_value = event.data().value().parse::<f64>();
        if input_value.is_ok() {
            let new_price = input_value.unwrap();
            price.set(new_price);
        }
    };
    // end region :  --- Handle Price Input

    // region :      --- Handle Update Member
    let mut handle_update_member = move |person: &Member, selected| {
        if selected == false {
            let update_members = exclude_member(&selected_member(), &person.name);
            selected_member.set(update_members);
        } else {
            selected_member.push(person.clone());
        }
    };
    // end region :  --- Handle Update Member

    // region :      --- Handle Select All Member
    let handle_select_all = move |_| {
        let all_members = context.read().members.clone();
        selected_member.set(all_members.to_owned());
    };
    // end region :  --- Handle Select All Member

    // region :      --- Handle Unselect Member
    let handle_un_select_all = move |_| {
        selected_member.set(Vec::new());
    };
    // end region :  --- Handle Unselect Member

    // region :      --- Handle Add Order
    let handle_add_order = move |_| {
        let id = Uuid::new_v4().to_string();
        let new_order = Order {
            id,
            title: title(),
            price: price(),
            members: selected_member(),
        };
        context.write().orders.push(new_order);
        // Reset input to default value
        title.set(String::from(""));
        selected_member.set(Vec::new());
        price.set(0.0);
    };
    // end region :  --- Handle Add Order

    rsx! {
        div {
            class: "px-4 md:px-2 space-y-12",
            div {
                class: "pt-4 pb-4",
                div {
                    class: "grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-12",
                    div {
                        class: "sm:col-span-6",
                        label {
                            class: "block text-sm/6 font-medium text-gray-900",
                            for: "order-name",
                            "ชื่อรายการ"
                        }
                        div {
                            class: "mt-2",
                            OrderInput {
                                name: title,
                                oninput: move |event: FormEvent| title.set(event.value())
                            }
                        }
                    }
                    div {
                        class: "sm:col-span-6",
                        label {
                            class: "block text-sm/6 font-medium text-gray-900",
                            for: "order-price",
                            "ราคา"
                        }
                        div {
                            class: "mt-2 flex items-center rounded-md bg-white px-3 outline-1 -outline-offset-1 outline-gray-300 focus-within:outline-2 focus-within:-outline-offset-2 focus-within:outline-blue-500",
                            OrderPriceInput {
                                value: price(),
                                oninput: handle_price_change
                            }
                        }
                    }
                    div {
                        class: "sm:col-span-12",
                        fieldset {
                            legend {
                                class: "text-base font-semibold text-gray-900",
                                "คนจ่าย"
                            }

                            div {
                                for person in read_context.members.clone() {
                                    OrderMemberCheckBox {
                                        name: "{person.name}",
                                        selected: selected_member.iter().find(|m| m.name == person.name).is_some(),
                                        onselect: move |selected| handle_update_member(&person, selected),
                                    }
                                }

                                if read_context.members.len() == 0 {
                                    div {
                                        class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                                        "ยังไม่มีคนจ่าย"
                                    }
                                } else {
                                    div {
                                        class: "relative inline-flex gap-3 py-2 mr-2",
                                        if selected_member.len() == read_context.members.len() {
                                            button {
                                                r#type: "button",
                                                class: "cursor-pointer inline-flex items-center gap-x-1.5 rounded-md p-2 text-xs font-medium bg-red-400 hover:bg-red-500 text-white",
                                                onclick: handle_un_select_all,
                                                "ยกเลิกการเลือก"
                                            }
                                        } else {
                                            button {
                                                r#type: "button",
                                                class: "cursor-pointer inline-flex items-center gap-x-1.5 rounded-md p-2 text-xs font-medium bg-sky-400 hover:bg-sky-500 text-white",
                                                onclick: handle_select_all,
                                                "เลือกทุกคน"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "sm:col-span-12",
                        button {
                            r#type: "button",
                            class: "cursor-pointer w-full rounded-md bg-blue-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-xs hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600",
                            onclick: handle_add_order,
                            "เพิ่มรายการ"
                        }
                    }
                }
            }
        }
    }
}

// TODO: move to helper
fn exclude_member(members: &Members, name: &str) -> Members {
    let tmp = members.clone();
    tmp.iter()
        .filter(|&m| m.name != name)
        .map(|m| m.clone())
        .collect()
}
