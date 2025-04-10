use dioxus::prelude::*;

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
            OrderModalEdit { }
        }
    }
}
