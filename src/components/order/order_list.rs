use dioxus::prelude::*;

use crate::{components::order::order_record::OrderRecord, state::app_state::AppState};

#[component]
pub fn OrderList() -> Element {
    let context = use_context::<Signal<AppState>>();

    rsx! {
        div {
            class: "p-2",
            ul {
                role: "list",
                class: "divide-y divide-gray-500/5",

                if context.read().orders.len() == 0 {
                    div {
                        class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                        "ยังไม่รายการ"
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
