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
        context.write().view = String::from("orders");
    };

    let handle_member_click = move |event: Event<MouseData>| {
        event.prevent_default();
        context.write().view = String::from("members");
    };
    // end region :  --- Handle Switch Tab

    rsx! {
        div {
            class: "block",
            div {
                class: "border-b border-gray-200",
                nav {
                    "aria-label": "Tabs",
                    class: "-mb-px flex space-x-8",
                    button {
                        r#type: "button",
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
                    button {
                        r#type: "button",
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
