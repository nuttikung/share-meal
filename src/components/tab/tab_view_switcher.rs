use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;

use crate::state::app_state::AppState;

#[component]
pub fn TabViewSwitcher() -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let current_tab_value = &context.read().view;

    // region :      --- Handle Switch Tab
    let handle_order_click = move |event: Event<MouseData>| {
        event.prevent_default();
        context.write().view = "orders".to_string();
    };

    let handle_member_click = move |event: Event<MouseData>| {
        event.prevent_default();
        context.write().view = "members".to_string();
    };

    let handle_select_tab = move |event: Event<FormData>| {
        let new_tab = event.data().value();
        context.write().view = new_tab;
    };
    // end region :  --- Handle Switch Tab

    rsx! {
        div {
            div { class: "grid grid-cols-1 sm:hidden ml-2 mr-2",
                select {
                    onchange: handle_select_tab,
                    // onselect: handle_select_tab,
                    aria_label: "Select a tab",
                    class: "col-start-1 row-start-1 w-full appearance-none rounded-md bg-white py-2 pr-8 pl-3 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 focus:outline-2 focus:-outline-offset-2 focus:outline-blue-500",
                    option {
                        value: "orders",
                        selected: current_tab_value.to_string() == "orders".to_string(),
                        "รายการ"
                    }
                    option {
                        value: "members",
                        selected: current_tab_value.to_string() == "members".to_string(),
                        "คนจ่าย"
                    }
                }
            }

            div {
                class: "hidden sm:block",
                div {
                    class: "border-b border-gray-200",
                    nav { "aria-label": "Tabs", class: "-mb-px flex space-x-8",
                        a {
                            href: "#",
                            onclick: handle_order_click,
                            class: "group inline-flex flex-1 justify-center items-center border-b-2 border-transparent px-1 py-4 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700",
                            class: if current_tab_value.to_string() == "orders".to_string() { "border-blue-400! text-blue-500!" },
                            span {
                                class: "mr-2 -ml-0.5 size-5 text-gray-400 group-hover:text-gray-500",
                                class: if current_tab_value.to_string() == "orders".to_string() { "border-blue-400! text-blue-500!" },
                                MaterialIcon {
                                    size: 20,
                                    name: "list"
                                }
                            }
                            span { "รายการ" }
                        }
                        a {
                            href: "#",
                            onclick: handle_member_click,
                            class: "group inline-flex flex-1 justify-center items-center border-b-2 border-transparent px-1 py-4 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700",
                            class: if current_tab_value.to_string() == "members".to_string() { "border-blue-400! text-blue-500!" },
                            span {
                                class: "mr-2 -ml-0.5 size-5 text-gray-400 group-hover:text-gray-500",
                                class: if current_tab_value.to_string() == "members".to_string() { "border-blue-400! text-blue-500!" },
                                MaterialIcon {
                                    size: 20,
                                    name: "people"
                                }
                            }
                            span { "คนจ่าย" }
                        }
                    }
                }
            }
        }
    }
}
