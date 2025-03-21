use dioxus::prelude::*;

use crate::{components::member::helper::is_member_exist, state::app_state::AppState};

#[component]
pub fn MemberInput() -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let mut person = use_signal(|| "".to_string());

    // region :      --- Handle Input Change
    let handle_person_input_change = move |event: Event<FormData>| {
        person.set(event.value());
    };
    // end region :  --- Handle Input Change

    // region :      --- Handle Add Person
    let handle_add_person = move |_| {
        if person.read().is_empty() {
            return;
        }

        let current_members_count = &context.read().members.len();
        if !is_member_exist(&context.read().members, &person.read()) {
            context
                .write()
                .members
                .insert(*current_members_count, person.to_string());
        }
        person.set("".to_string());
    };
    // end region :  --- Handle Add Person

    rsx! {
        div {
            class: "member-input-container",

            input {
                class: "input",
                placeholder: "ระบุชื่อ",
                value: "{person}",
                autofocus: "true",
                oninput: handle_person_input_change
                // onkeydown
            }

            button {
                class: "input",
                onclick: handle_add_person,
                "Add"
            }
        }
    }
}
