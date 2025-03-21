use dioxus::prelude::*;

use crate::state::app_state::AppState;

#[component]
pub fn MemberClearButton() -> Element {
    let mut context = use_context::<Signal<AppState>>();

    // region :      --- Handle Clear Members
    let handle_clear_members = move |_| {
        context.write().members.clear();
    };
    // end region :  --- Handle Clear Members

    rsx! {
        div {
            class: "clear-container",
            button {
                class: "danger",
                onclick: handle_clear_members,
                "ล้างรายชื่อทั้งหมด"
            }
        }
    }
}
