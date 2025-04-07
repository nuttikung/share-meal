use dioxus::prelude::*;

use crate::state::{
    app_state::AppState,
    order::{Order, Orders},
};

#[component]
pub fn MemberClearButton() -> Element {
    let mut context = use_context::<Signal<AppState>>();

    // region :      --- Handle Clear Members
    let handle_clear_members = move |_| {
        let orders = context.read().orders.clone();
        let order_without_member: Orders = orders
            .clone()
            .iter()
            .map(|o| make_order_empty_members(o))
            .collect();
        context.write().members.clear();
        context.write().orders = order_without_member;
    };
    // end region :  --- Handle Clear Members

    rsx! {
        div {
            class: "flex justify-center items-center my-2 pb-2",
            button {
                r#type: "button",
                class: "rounded-md bg-red-50 px-2.5 py-1.5 text-sm font-semibold text-red-600 shadow-xs hover:bg-red-100",
                onclick: handle_clear_members,
                "ล้างรายชื่อทั้งหมด"
            }
        }
    }
}

/// Make the order struct to empter members.
fn make_order_empty_members(order: &Order) -> Order {
    return Order {
        id: order.id.to_string(),
        title: order.title.to_string(),
        price: order.price,
        members: vec![],
    };
}
