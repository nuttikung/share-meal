use dioxus::prelude::*;

#[component]
pub fn OrderInput() -> Element {
    rsx! {
        div {
            class: "order-input-container",

            div {
                class: "order-input",
                input {
                    placeholder: "ระบุรายการ",
                    // value: "{person}",
                    autofocus: "true",
                    // oninput: handle_person_input_change
                    // onkeydown
                }
            }

            div {
                class: "order-input",
                input {
                    placeholder: "ราคา",
                    // value: "{person}",
                    autofocus: "true",
                    // oninput: handle_person_input_change
                    // onkeydown
                }
            }

            // Add Member to share

            button {
                // class: "input",
                // onclick: handle_add_person,
                "Add"
            }

            // Calculator here
        }
    }
}
