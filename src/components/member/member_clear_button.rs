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
            class: "flex justify-center items-center my-2 pb-2",
            button {
                type: "button",
                class: "rounded-md bg-red-50 px-2.5 py-1.5 text-sm font-semibold text-red-600 shadow-xs hover:bg-red-100",
                onclick: handle_clear_members,
                "ล้างรายชื่อทั้งหมด"
            }
        }
    }
}
