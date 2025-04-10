use dioxus::prelude::*;

use crate::{components::order::order_record::OrderRecord, state::app_state::AppState};

#[component]
pub fn OrderList() -> Element {
    let context = use_context::<Signal<AppState>>();
    let orders = &context.read().orders;
    let seleted_order = &context.read().seleted_order;
    let show_modal: bool = seleted_order.is_some();

    rsx! {
        div {
            class: "py-2 px-4 md:px-2",
            ul {
                role: "list",
                class: "divide-y divide-gray-500/5",

                if orders.len() == 0 {
                    div {
                        class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                        "ยังไม่รายการ"
                    }
                }

                for order in orders {
                    OrderRecord {
                        id: "{order.id}",
                        title: "{order.title}",
                        price: order.price,
                        members: order.members.clone()
                    }
                }
            }
        }

        if show_modal {
            div {
                "aria-labelledby": "modal-title",
                role: "dialog",
                "aria-modal": "true",
                class: "relative z-10",
                div {
                    "aria-hidden": "true",
                    class: "fixed inset-0 bg-gray-500/75 transition-opacity",
                }
                div { class: "fixed inset-0 z-10 w-screen overflow-y-auto",
                    div { class: "flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0",
                        div { class: "relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6",
                            div {
                                div { class: "mx-auto flex size-12 items-center justify-center rounded-full bg-green-100",
                                    svg {
                                        "stroke-width": "1.5",
                                        stroke: "currentColor",
                                        "aria-hidden": "true",
                                        "viewBox": "0 0 24 24",
                                        "data-slot": "icon",
                                        fill: "none",
                                        class: "size-6 text-green-600",
                                        path {
                                            "stroke-linecap": "round",
                                            d: "m4.5 12.75 6 6 9-13.5",
                                            "stroke-linejoin": "round",
                                        }
                                    }
                                }
                                div { class: "mt-3 text-center sm:mt-5",
                                    h3 {
                                        class: "text-base font-semibold text-gray-900",
                                        id: "modal-title",
                                        "Payment successful"
                                    }
                                    div { class: "mt-2",
                                        p { class: "text-sm text-gray-500",
                                            "Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequatur amet labore."
                                        }
                                    }
                                }
                            }
                            div { class: "mt-5 sm:mt-6",
                                button {
                                    r#type: "button",
                                    class: "inline-flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                                    "Go back to dashboard"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
