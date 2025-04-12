use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;

use crate::{components::member::member_record::MemberRecord, state::app_state::AppState};

#[component]
pub fn MemberList() -> Element {
    let context = use_context::<Signal<AppState>>();

    rsx! {
        div {
            class: "px-4 md:px-2 py-3",
            if context.read().members.len() == 0 {
                div {
                    class: "relative block w-full rounded-lg border-2 border-dashed border-gray-300 p-12 text-center hover:border-gray-400 focus:ring-2 focus:ring-offset-2 focus:outline-hidden",
                    span {
                        class: "text-gray-400",
                        MaterialIcon {
                            name: "groups",
                            size: 48,
                        }
                    }
                    span {
                        class: "mt-2 block text-sm font-semibold text-gray-900",
                        "ยังไม่มีคนจ่าย"
                    }
                }
            } else {
                ul {
                    role: "list",
                    class: "grid grid-cols-1 gap-4 sm:grid-cols-2",
                    for person in &context.read().members {
                        MemberRecord {
                            key: "{person.name}",
                            name: "{person.name}",
                            paid: person.paid
                        }
                    }
                }
            }
        }
    }
}
