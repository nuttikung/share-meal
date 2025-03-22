use dioxus::prelude::*;

use crate::{
    components::member::helper::is_member_exist,
    state::{app_state::AppState, member::Member},
};

#[component]
pub fn MemberInput() -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let mut person = use_signal(|| "".to_string());

    // region :      --- Handle Input Change
    let handle_person_input_change = move |event: Event<FormData>| {
        person.set(event.value());
    };
    // end region :  --- Handle Input Change

    // region :      --- Add Member
    let mut add_member = move || {
        if person.read().is_empty() {
            return;
        }

        let current_members_count = &context.read().members.len();
        if !is_member_exist(&context.read().members, &person.read()) {
            let new_member = Member {
                name: person.to_string(),
                paid: false,
            };

            context
                .write()
                .members
                .insert(*current_members_count, new_member);
        }
        person.set("".to_string());
    };
    // end region :  --- Add Member

    // region :      --- Handle Add Person
    let handle_add_person = move |_| {
        add_member();
    };
    // end region :  --- Handle Add Person

    // region :      --- Handle Enter Press
    let handle_enter_press = move |event: Event<KeyboardData>| {
        if event.code().to_string() == "Enter".to_string() {
            add_member();
        }
    };
    // end region :  --- Handle Enter Press

    rsx! {
        div {
            class: "member-input-container",

            input {
                class: "input",
                placeholder: "ระบุชื่อ",
                value: "{person}",
                autofocus: "true",
                oninput: handle_person_input_change,
                onkeydown: handle_enter_press
            }

            button {
                class: "input",
                onclick: handle_add_person,
                "Add"
            }
        }
    }
}
