use dioxus::prelude::*;

#[component]
pub fn OrderInput() -> Element {
    rsx! {
        div {
            class: "member-input-container",

            input {
                class: "input",
                placeholder: "ระบุรายการ",
                // value: "{person}",
                autofocus: "true",
                // oninput: handle_person_input_change
                // onkeydown
            }

            button {
                class: "input",
                // onclick: handle_add_person,
                "Add"
            }
        }
    }
}
