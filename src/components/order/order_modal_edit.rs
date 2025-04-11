use crate::{
    components::order::{
        order_input::OrderInput, order_member_check_box::OrderMemberCheckBox,
        order_price_input::OrderPriceInput,
    },
    state::{
        app_state::AppState,
        member::{Member, Members},
    },
};
use dioxus::prelude::*;

#[component]
pub fn OrderModalEdit() -> Element {
    let mut context = use_context::<Signal<AppState>>();
    let read_context = context.read();
    let seleted_order = context.read().seleted_order.clone().unwrap();
    let mut title = use_signal(|| String::from(seleted_order.title));
    let mut price = use_signal(|| seleted_order.price);
    let mut members = use_signal(|| seleted_order.members.clone());

    // region :      --- Handle Price Input
    let handle_price_change = move |event: Event<FormData>| {
        let input_value = event.data().value().parse::<f64>();
        if input_value.is_ok() {
            let new_price = input_value.unwrap();
            price.set(new_price);
        }
    };
    // end region :  --- Handle Price Input

    // region :      --- Handle Update Member
    let mut handle_update_member = move |person: &Member, selected| {
        if selected == false {
            let update_members = exclude_member(&members(), &person.name);
            members.set(update_members);
        } else {
            members.push(person.clone());
        }
    };
    // end region :  --- Handle Update Member

    // region :      --- Handle Unselect Member
    let handle_un_select_all = move |_| {
        members.set(Vec::new());
    };
    // end region :  --- Handle Unselect Member

    // region :      --- Handle Select All Member
    let handle_select_all = move |_| {
        let all_members = context.read().members.clone();
        members.set(all_members.to_owned());
    };
    // end region :  --- Handle Select All Member

    // region :      --- Handle Update Order
    let handle_apply_order = move |_: Event<MouseData>| {
        let mut app_state = context.write();
        if let Some(order_to_update) = app_state
            .orders
            .iter_mut()
            .find(|o| o.id == seleted_order.id)
        {
            order_to_update.title = title();
            order_to_update.price = price();
            order_to_update.members = members();
        }
        // close modal
        app_state.seleted_order = None;
    };
    // end region :  --- Handle Update Order

    rsx! {
        div {
            "aria-labelledby": "modal-title",
            role: "dialog",
            "aria-modal": "true",
            class: "relative z-10",
            div {
                "aria-hidden": "true",
                class: "fixed inset-0 bg-gray-500/75 transition-opacity",
            }

            div {
                class: "fixed inset-0 z-10 w-screen overflow-y-auto",
                div {
                    class: "flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0",
                    div {
                        class: "relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 w-full sm:min-w-sm sm:max-w-sm sm:p-6",
                        div {
                            OrderInput {
                                name: title,
                                oninput: move |event: FormEvent| title.set(event.value())
                            }
                        }

                        div {
                            class: "mt-2 flex items-center rounded-md bg-white px-3 outline-1 -outline-offset-1 outline-gray-300 focus-within:outline-2 focus-within:-outline-offset-2 focus-within:outline-blue-500",
                            OrderPriceInput {
                                value: price(),
                                oninput: handle_price_change
                            }
                        }

                        div {
                            for person in read_context.members.clone() {
                                OrderMemberCheckBox {
                                    name: "{person.name}",
                                    selected: members.iter().find(|m| m.name == person.name).is_some(),
                                    onselect: move |selected| handle_update_member(&person, selected),
                                }
                            }

                            if read_context.members.len() == 0 {
                                div {
                                    class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                                    "ยังไม่มีคนจ่าย"
                                }
                            }   else {
                                div {
                                    class: "relative inline-flex gap-3 py-2 mr-2",

                                    if members.len() == read_context.members.len() {
                                        button {
                                            r#type: "button",
                                            class: "cursor-pointer inline-flex items-center gap-x-1.5 rounded-md p-2 text-xs font-medium bg-red-400 hover:bg-red-500 text-white",
                                            onclick: handle_un_select_all,
                                            "ยกเลิกการเลือก"
                                        }
                                    } else {
                                        button {
                                            r#type: "button",
                                            class: "cursor-pointer inline-flex items-center gap-x-1.5 rounded-md p-2 text-xs font-medium bg-sky-400 hover:bg-sky-500 text-white",
                                            onclick: handle_select_all,
                                            "เลือกทุกคน"
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "mt-5 sm:mt-6",
                            button {
                                class: "inline-flex w-full justify-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600",
                                r#type: "button",
                                onclick: handle_apply_order,
                                "บันทึก"
                            }
                        }
                    }
                }
            }
        }
    }
}

// TODO: move to helper
fn exclude_member(members: &Members, name: &str) -> Members {
    let tmp = members.clone();
    tmp.iter()
        .filter(|&m| m.name != name)
        .map(|m| m.clone())
        .collect()
}
