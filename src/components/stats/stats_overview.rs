use dioxus::prelude::*;

use crate::{
    helper::price::round_up_float_to_two_precision,
    state::{app_state::AppState, order::Order},
};

#[component]
pub fn StatsOverview() -> Element {
    let context = use_context::<Signal<AppState>>();
    let member_count = context.read().members.len();
    let orders = context.read().orders.clone();
    let total_price = calculate_total_price(orders);
    let total_price_round_up = round_up_float_to_two_precision(total_price);

    rsx! {
        dl {
            class: "mx-auto grid grid-cols-2 gap-px",
            div {
                class: "flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1 p-4",
                dt {
                    class: "text-sm/6 font-medium text-gray-500",
                    "จำนวนคน"
                }
                dd {
                    class: "w-full flex-none text-3xl/10 font-medium tracking-tight text-gray-900",
                    "{member_count}"
                }
            }
            div {
                class: "flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1 p-4",
                dt {
                    class: "text-sm/6 font-medium text-gray-500",
                    "ราคารวม"
                }
                dd {
                    class: "w-full flex-none text-3xl/10 font-medium tracking-tight text-gray-900",
                    "{total_price_round_up}"
                }
            }

        }
    }
}

fn calculate_total_price(orders: Vec<Order>) -> f64 {
    let mut total: f64 = 0.0;
    for o in orders {
        total += o.price as f64;
    }
    return total;
}
