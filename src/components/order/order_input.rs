use dioxus::prelude::*;

use crate::{components::order::order_member_check_box::OrderMemberCheckBox, state::app_state::AppState};

#[component]
pub fn OrderInput() -> Element {
    let context = use_context::<Signal<AppState>>();
    let all_members = context.read().members.clone();
    let mut order_name = use_signal(|| "".to_string());
    let mut price = use_signal(|| 0.0);
    let mut selected_member = use_signal(|| vec![] as Vec<String>);

    // region :      --- Handle Order Name Input
    let handle_order_name_change = move |event:Event<FormData>| {
        order_name.set(event.value());
    };
    // end region :  --- Handle Order Name Input

    // region :      --- Handle Price Input
    let handle_price_change = move |event:Event<FormData>| {
        let input_value = event.data().value().parse::<f64>();
        // Error
        if input_value.is_err() {
            return
        }

        let new_price = input_value.unwrap();
        // Price should not below 0
        if new_price.is_sign_negative() {
            return
        }

        price.set(new_price);
    };
    // end region :  --- Handle Price Input

    rsx! {
        div {
            class: "m-2 space-y-12",
            div {
                class: "pt-4 pb-4",
                div {
                    class: "grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6",
                    div {
                        class: "sm:col-span-3",
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
                        class: "sm:col-span-3",
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
                                class: "mt-4 divide-y divide-gray-200 border-t border-b border-gray-200",
                                for person in all_members {
                                    OrderMemberCheckBox {
                                        name: "{person.name}",
                                        selected: !selected_member.read().iter().find(|m| **m == person.name).is_none(),
                                        onselect: move |event:Event<FormData>| {
                                            // Implement Add and Remove Shared Member here.
                                            if event.data().value() == "true".to_string() {
                                                selected_member.write().push(person.name.to_string());
                                            } else {
                                                // selected_member.write().remove(index)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
