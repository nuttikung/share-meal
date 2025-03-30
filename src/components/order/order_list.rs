use dioxus::prelude::*;

use crate::{components::order::order_record::OrderRecord, state::app_state::AppState};

#[component]
pub fn OrderList() -> Element {
    let context = use_context::<Signal<AppState>>();

    rsx! {
        div {
            class: "sm:px-6 lg:px-8 border-b border-gray-900/10 py-2",
            div {
                class: "m-2 flow-root",
                div {
                    class: "overflow-x-auto sm:-mx-6 lg:-mx-8",
                    div {
                        class: "inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8",
                        table {
                            class: "min-w-full",
                            thead {
                                tr {
                                    th {
                                        scope: "col",
                                        class: "py-3.5 pr-3 pl-4 text-left text-sm font-semibold text-gray-900 sm:pl-0",
                                        "ชื่อรายการ"
                                    }
                                    th {
                                        scope: "col",
                                        class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                                        "ราคา"
                                    }
                                    th {
                                        scope: "col",
                                        class: "px-3 py-3.5 text-center text-sm font-semibold text-gray-900",
                                        "คนละ"
                                    }
                                }
                            }

                            tbody {
                                if context.read().orders.len() == 0{
                                    tr {
                                        td {
                                            class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                                            colspan: 3,
                                            "ยังไม่รายการ"
                                        }
                                    }
                                }

                                for order in &context.read().orders {
                                    OrderRecord {
                                        id: "{order.id}",
                                        title: "{order.title}",
                                        price: order.price,
                                        members: order.members.clone()
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
