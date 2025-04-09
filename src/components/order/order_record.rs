use dioxus::{logger::tracing, prelude::*};
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
        members:members.clone(),
    };
    let price_per_member = order.calculate_price_per_member();
    let round_up_price = round_up_float_to_two_precision(price_per_member);


    // region :      --- Handle Remove Order
    // let handle_remove_order = move |_| {
    //     let orders = context.read().orders.clone();
    //     let updated_order: Orders = orders
    //         .clone()
    //         .iter()
    //         .filter(|o| o.id != order.id)
    //         .map(|o| o.clone())
    //         .collect();
    //     context.write().orders = updated_order;
    // };
    // end region :  --- Handle Remove Order

    rsx!(
        li {
            class: "relative flex items-center space-x-4 py-4",
            div {
                class: "min-w-0 flex-auto",
                div {
                    class: "flex items-center gap-x-3",
                    h2 {
                        class: "min-w-0 text-sm/6 font-semibold",
                        a {
                            href: "#",
                            class: "flex gap-x-2",
                            span {
                                class: "truncate",
                                "{title}"
                            }
                            span {
                                class: "absolute inset-0"
                            }
                        }
                    }
                }

                div {
                    class: "py-1 text-xs font-medium text-gray-400",
                    "ราคา {price} (คนละ {round_up_price})"
                }

                div {
                    class: "py-1 flex items-center gap-x-2.5 text-xs/5 text-gray-400",

                    if members.clone().len() == 0 {
                        div {
                            class: "text-sm text-center whitespace-nowrap text-gray-500",
                            "ยังไม่คนจ่าย"
                        }
                    }

                    for m in members.clone() {
                        span { class: "inline-flex items-center gap-x-0.5 rounded-md bg-gray-50 px-2 py-1 text-xs font-medium text-gray-600 ring-1 ring-gray-500/10 ring-inset",
                            "{m.name}"
                            button {
                                r#type: "button",
                                class: "group relative -mr-1 size-3.5 rounded-xs hover:bg-gray-500/20",
                                // span { class: "sr-only", "Remove" }
                                // svg {
                                //     "viewBox": "0 0 14 14",
                                //     class: "size-3.5 stroke-gray-600/50 group-hover:stroke-gray-600/75",
                                //     path { d: "M4 4l6 6m0-6l-6 6" }
                                // }
                                // span { class: "absolute -inset-1" }

                                // onclick: handle_remove_order,
                                MaterialIcon {
                                    size: 14,
                                    name: "close"
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "size-5 flex-none text-gray-400",
                MaterialIcon {
                    size: 20,
                    name: "chevron_right"
                }
            }
        }

        // tr {
        //     td {
        //         class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
        //         "{title}"

        //         div {
        //             class: "flex gap-3",

        //             for m in members.clone() {
        //                 span { class: "inline-flex items-center gap-x-0.5 rounded-md bg-gray-50 px-2 py-1 text-xs font-medium text-gray-600 ring-1 ring-gray-500/10 ring-inset",
        //                     "{m.name}"
        //                     button {
        //                         r#type: "button",
        //                         class: "group relative -mr-1 size-3.5 rounded-xs hover:bg-gray-500/20",
        //                         // span { class: "sr-only", "Remove" }
        //                         // svg {
        //                         //     "viewBox": "0 0 14 14",
        //                         //     class: "size-3.5 stroke-gray-600/50 group-hover:stroke-gray-600/75",
        //                         //     path { d: "M4 4l6 6m0-6l-6 6" }
        //                         // }
        //                         // span { class: "absolute -inset-1" }

        //                         MaterialIcon {
        //                             size: 14,
        //                             name: "close"
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     td {
        //         class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
        //         "{price}"
        //     }
        //     td {
        //         class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
        //         "{round_up_price}"
        //     }
        //     td {
        //         class: "px-3 py-4 flex gap-4 text-sm text-center whitespace-nowrap text-gray-500",
        //         button {
        //             r#type: "button",
        //             class: "cursor-pointer rounded-full flex bg-gray-400 p-1.5 text-gray-100 shadow-xs hover:gray-200 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:gray-200",
        //             // onclick: handle_remove_order,
        //             MaterialIcon {
        //                 size: 20,
        //                 name: "mode_edit"
        //             }
        //         }

        //         button {
        //             r#type: "button",
        //             class: "cursor-pointer rounded-full flex bg-red-50 p-1.5 text-red-600 shadow-xs hover:bg-red-100 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-100",
        //             onclick: handle_remove_order,
        //             MaterialIcon {
        //                 size: 20,
        //                 name: "close"
        //             }
        //         }
        //     }
        // }
    )
}
