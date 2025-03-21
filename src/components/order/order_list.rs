use dioxus::prelude::*;

#[component]
pub fn OrderList() -> Element {
    rsx! {
        div {
            id: "overview",
            "this is order list view"
        }
    }
}
