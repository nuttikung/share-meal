use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;

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
            class: "m-2 py-2 flex",
            div {
                class: "-mr-px grid grow grid-cols-1 focus-within:relative",
                input {
                    class: "col-start-1 row-start-1 block w-full rounded-l-md bg-white py-1.5 pr-3 pl-10 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-blue-500 sm:pl-9 sm:text-sm/6",
                    aria_label: "name",
                    type: "name",
                    placeholder: "ระบุชื่อ",
                    value: "{person}",
                    autocomplete: "off",
                    oninput: handle_person_input_change,
                    onkeydown: handle_enter_press
                }
                span {
                    class: "pointer-events-none col-start-1 row-start-1 ml-3 size-5 self-center text-gray-400 sm:size-4",
                    MaterialIcon {
                        name: "person",
                        size: 16,
                    }
                }
            }
            button {
                r#type: "button",
                class: "cursor-pointer flex shrink-0 items-center gap-x-1.5 rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 outline-1 -outline-offset-1 outline-gray-300 hover:bg-gray-50 focus:relative focus:outline-2 focus:-outline-offset-2 focus:outline-blue-500",
                onclick: handle_add_person,
                span {
                    class: "pointer-events-none flex col-start-1 row-start-1 ml-3 mr-3 size-5 self-center items-center text-gray-400 sm:size-4",
                    MaterialIcon {
                        name: "add",
                        size: 20,
                    }
                }
            }
        }
    }
}
