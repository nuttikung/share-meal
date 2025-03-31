use dioxus::prelude::*;

use crate::state::member::Members;

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
        id: _,
        title,
        price,
        members,
    } = props;

    let price_per_one = price / members.len() as f64;
    let round_up_price = format!("{:.2}", price_per_one);

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
