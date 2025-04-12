use dioxus::{html::col::span, prelude::*};
use dioxus_material_icons::MaterialIcon;

use crate::{
    components::order::{order_modal_edit::OrderModalEdit, order_record::OrderRecord},
    state::app_state::AppState,
};

#[component]
pub fn OrderList() -> Element {
    let context = use_context::<Signal<AppState>>();
    let orders = &context.read().orders;
    let seleted_order = &context.read().seleted_order;
    let show_modal: bool = seleted_order.is_some();

    rsx! {
        div {
            class: "px-4 md:px-2 py-3",
            ul {
                role: "list",
                class: "divide-y divide-gray-500/5",

                if orders.len() == 0 {
                    div {
                        class: "relative block w-full rounded-lg border-2 border-dashed border-gray-300 p-12 text-center hover:border-gray-400 focus:ring-2 focus:ring-offset-2 focus:outline-hidden",
                        span {
                            class: "text-gray-400",
                            MaterialIcon {
                                name: "payments",
                                size: 48,
                            }
                        }
                        span {
                            class: "mt-2 block text-sm font-semibold text-gray-900",
                            "ยังไม่มีรายการ"
                        }
                    }
                } else {
                    for order in orders {
                        OrderRecord {
                            key: "{order.id}",
                            id: "{order.id}",
                            title: "{order.title}",
                            price: order.price,
                            members: order.members.clone()
                        }
                    }
                }
            }
        }

        if show_modal {
            OrderModalEdit { }
        }
    }
}
