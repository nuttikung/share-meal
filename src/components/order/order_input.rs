use dioxus::prelude::*;

use crate::{
    components::order::order_member_check_box::OrderMemberCheckBox, state::app_state::AppState,
};

#[component]
pub fn OrderInput() -> Element {
    let context = use_context::<Signal<AppState>>();
    let all_members = context.read().members.clone();
    let mut order_name = use_signal(|| "".to_string());
    let mut price = use_signal(|| 0.0);
    let mut selected_member = use_signal(|| vec![] as Vec<String>);

    // region :      --- Handle Order Name Input
    let handle_order_name_change = move |event: Event<FormData>| {
        order_name.set(event.value());
    };
    // end region :  --- Handle Order Name Input

    // region :      --- Handle Price Input
    let handle_price_change = move |event: Event<FormData>| {
        let input_value = event.data().value().parse::<f64>();
        // Error
        if input_value.is_err() {
            return;
        }

        let new_price = input_value.unwrap();
        // Price should not below 0
        if new_price.is_sign_negative() {
            return;
        }

        price.set(new_price);
    };
    // end region :  --- Handle Price Input

    // region :      --- Handle Select All Member
    let handle_select_all = move |_| {
        let members: Vec<String> = context
            .read()
            .members
            .clone()
            .iter()
            .map(|m| m.name.clone())
            .collect();
        if members.len() == selected_member.read().len() {
            selected_member.set(Vec::new());
        } else {
            selected_member.set(members);
        }
    };
    // end region :  --- Handle Select All Member

    rsx! {
        div {
            class: "m-2 space-y-12",
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
                            input {
                                id: "order-name",
                                class: "block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-blue-500 sm:text-sm/6",
                                autocomplete: "off",
                                type: "text",
                                name: "order_name",
                                value: "{order_name}",
                                onchange: handle_order_name_change,
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
                            div {
                                class: "shrink-0 text-base text-gray-500 select-none sm:text-sm/6",
                                "฿"
                            }
                            input {
                                id: "order-price",
                                class: "block min-w-0 grow py-1.5 pr-3 pl-1 text-base text-gray-900 placeholder:text-gray-400 focus:outline-none sm:text-sm/6",
                                placeholder: "0.00",
                                type: "number",
                                name: "price",
                                aria_describedby: "price-currency",
                                value: "{price}",
                                onchange: handle_price_change
                            }
                            div {
                                id: "price-currency",
                                class: "shrink-0 text-base text-gray-500 select-none sm:text-sm/6",
                                "บาท"
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
                                for person in all_members.clone() {
                                    OrderMemberCheckBox {
                                        name: "{person.name}",
                                        selected: !selected_member.read().iter().find(|m| **m == person.name).is_none(),
                                        onselect: move |new_value| {
                                            if new_value {
                                                selected_member.write().push(person.name.to_string());
                                            } else {
                                                let index = selected_member.read().iter().position(|m| *m == person.name).unwrap();
                                                selected_member.write().remove(index);
                                            }
                                        }
                                    }
                                }
                            }

                            if all_members.len() == 0 {
                                div {
                                    class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                                    "ยังไม่มีคนจ่าย"
                                }
                            } else {
                                button {
                                    r#type: "button",
                                    class: "cursor-pointer rounded-md bg-blue-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-xs hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600",
                                    onclick: handle_select_all,
                                    "เลือกทุกคน"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
