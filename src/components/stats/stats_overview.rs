use dioxus::prelude::*;

use crate::state::app_state::AppState;

#[component]
pub fn StatsOverview() -> Element {
    let context = use_context::<Signal<AppState>>();
    let member_count = context.read().members.len();

    rsx! {
        dl {
            class: "mx-auto grid grid-cols-2 gap-px",
            div {
                class: "flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1 p-4",
                dt { class: "text-sm/6 font-medium text-gray-500", "จำนวนคน" }
                dd { class: "w-full flex-none text-3xl/10 font-medium tracking-tight text-gray-900",
                    "{member_count}"
                }
            }
            div {
                class: "flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1 p-4",
                dt { class: "text-sm/6 font-medium text-gray-500", "ราคารวม" }
                dd { class: "w-full flex-none text-3xl/10 font-medium tracking-tight text-gray-900",
                    "0"
                }
            }

        }
    }
}
