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
        members,
    };
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

    rsx!(
        tr {
            td {
                class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                "{title}"
            }
            td {
                class: "px-3 py-4 text-sm whitespace-nowrap text-gray-500",
                "{price}"
            }
            td {
                class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                "{round_up_price}"
            }
            td {
                class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                button {
                    r#type: "button",
                    class: "cursor-pointer rounded-full flex bg-red-50 p-1.5 text-red-600 shadow-xs hover:bg-red-100 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-100",
                    onclick: handle_remove_order,
                    MaterialIcon {
                        size: 20,
                        name: "close"
                    }
                }
            }
        }
    )
}
