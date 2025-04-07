use dioxus::prelude::*;

use crate::{helper::price::round_up_float_to_two_precision, state::{member::Members, order::Order}};

#[derive(PartialEq, Props, Clone)]
pub struct OrderRecordProps {
    id: String,
    title: String,
    price: f64,
    members: Members,
}

#[component]
pub fn OrderRecord(props: OrderRecordProps) -> Element {
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
        members
    };
    let price_per_member = order.calculate_price_per_member();
    let round_up_price = round_up_float_to_two_precision(price_per_member);

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
        }
    )
}
