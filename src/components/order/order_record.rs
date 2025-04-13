use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;

use crate::{
    helper::price::round_up_float_to_two_precision,
    state::{
        app_state::AppState,
        member::Members,
        order::{Order, Orders},
    },
};

#[derive(PartialEq, Props, Clone)]
pub struct OrderRecordProps {
    id: String,
    title: String,
    price: f64,
    members: Members,
}

#[component]
pub fn OrderRecord(props: OrderRecordProps) -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let OrderRecordProps {
        id,
        title,
        price,
        members,
    } = props;

    let order = Order {
        id: id.clone(),
        title: title.clone(),
        price,
        members: members.clone(),
    };

    let members = &order.members;
    let price_per_member = order.calculate_price_per_member();
    let round_up_price = round_up_float_to_two_precision(price_per_member);

    // region :      --- Handle Remove Order
    let handle_remove_order = move |_| {
        let orders = context.read().orders.clone();
        let updated_order: Orders = orders
            .clone()
            .iter()
            .filter(|o| o.id != order.id)
            .map(|o| o.clone())
            .collect();
        context.write().orders = updated_order;
    };
    // end region :  --- Handle Remove Order

    // region :      --- Handle Edit Order
    let handle_edit_order = move |_| {
        let orders = context.read().orders.clone();
        let order = orders.iter().find(|&o| o.id == id.clone());

        if order.is_none() {
            context.write().seleted_order = None;
            return;
        }

        let selected_order = order.unwrap().clone();
        context.write().seleted_order = Some(selected_order);
    };
    // end region :  --- Handle Edit Order

    rsx!(
        li {
            div {
                class: "group flex w-full items-center justify-between rounded-xl border border-gray-300 p-2 text-left shadow-xs hover:bg-gray-50 focus:ring-2 focus:ring-offset-2 focus:outline-hidden",
                span {
                    class: "flex min-w-0 flex-1 items-center space-x-3",
                    span {
                        class: "block min-w-0 flex-1",
                        span {
                            class: "block truncate text-sm font-medium text-gray-900 mb-1",
                            if &title == "" {
                                "ยังไม่ได้ระบุชื่อรายการ"
                            } else {
                                "{title}"
                            }
                        }

                        div {
                            class: "block truncate text-sm font-medium text-gray-500 mb-1",
                            span {
                                class: "block truncate",
                                "ราคา {price}"
                            }
                            span {
                                class: "block truncate",
                                "คนละ {round_up_price}"
                            }
                        }

                        div {
                            class: "flex flex-wrap space-x-1",
                            if members.len() == 0 {
                                div {
                                    class: "text-sm whitespace-nowrap text-gray-500",
                                    "ยังไม่มีคนจ่าย"
                                }
                            }

                            for m in &members {
                                span {
                                    key: "{m.name}",
                                    class: "inline-flex items-center gap-x-0.5 rounded-md bg-gray-50 px-2 py-1 my-1 text-xs font-medium text-gray-600 ring-1 ring-gray-500/10 ring-inset",
                                    "{m.name}"
                                }
                            }
                        }
                    }
                }

                div {
                    class: "inline-flex size-10 shrink-0 items-center justify-center",
                    button {
                        class: "text-gray-400",
                        r#type: "button",
                        onclick: handle_edit_order,
                        MaterialIcon {
                            size: 20,
                            name: "edit"
                        }
                    }
                }

                div {
                    class: "inline-flex size-10 shrink-0 items-center justify-center",
                    button {
                        r#type: "button",
                        class: "text-red-400",
                        onclick: handle_remove_order,
                        MaterialIcon {
                            size: 20,
                            name: "delete"
                        }
                    }
                }
            }
        }

        // li {
        //     class: "relative flex items-center space-x-1 py-4",
        //     div {
        //         class: "min-w-0 flex-auto",
        //         div {
        //             class: "flex items-center gap-x-3",
        //             h2 {
        //                 class: "min-w-0 text-sm/6 font-semibold",
        //                 div {
        //                     class: "flex gap-x-2",
        //                     span {
        //                         class: "truncate",
        //                         if &title == "" {
        //                             "ยังไม่ได้ระบุชื่อรายการ"
        //                         } else {
        //                             "{title}"
        //                         }
        //                     }
        //                 }
        //             }
        //         }

        //         div {
        //             class: "py-1 text-xs font-medium text-gray-400",
        //             "ราคา {price} (คนละ {round_up_price})"
        //         }

        //         div {
        //             class: "py-1 flex flex-wrap items-center gap-x-1.5 text-xs/5 text-gray-400",

        //             if members.len() == 0 {
        //                 div {
        //                     class: "text-sm text-center whitespace-nowrap text-gray-500",
        //                     "ยังไม่มีคนจ่าย"
        //                 }
        //             }

        //             for m in &members {
        //                 span {
        //                     key: "{m.name}",
        //                     class: "inline-flex items-center gap-x-0.5 rounded-md bg-gray-50 px-2 py-1 text-xs font-medium text-gray-600 ring-1 ring-gray-500/10 ring-inset",
        //                     "{m.name}"
        //                 }
        //             }
        //         }
        //     }

        //     div {
        //         class: "size-5 flex text-gray-400",
        //         button {
        //             r#type: "button",
        //             onclick: handle_edit_order,
        //             MaterialIcon {
        //                 size: 20,
        //                 name: "edit"
        //             }
        //         }
        //     }

        //     div {
        //         class: "size-5 flex text-gray-400",
        //         button {
        //             r#type: "button",
        //             class: "text-red-400",
        //             onclick: handle_remove_order,
        //             MaterialIcon {
        //                 size: 20,
        //                 name: "delete"
        //             }
        //         }
        //     }
        // }
    )
}
