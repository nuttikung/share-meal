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
use dioxus::{logger::tracing, prelude::*};

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
            return
        }



        // let update_members = members().iter_mut(|m| {
        //     if selected {
        //         m.push(person.clone());
        //     } else {
        //         if let Some(index) = members.iter().position(|m| m.name == person.name) {
        //             members.remove(index);
        //         }
        //     }
        // });

        // tracing::debug!("{:?} {:?}",selected, update_members);

        // members.set(update_members);
    };
    // end region :  --- Handle Update Member

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
                        class: "relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6",
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

                            // if members.len() == 0 {
                            //     div {
                            //         class: "px-3 py-4 text-sm text-center whitespace-nowrap text-gray-500",
                            //         "ยังไม่มีคนจ่าย"
                            //     }
                            // } else {
                            //     div {
                            //         class: "relative inline-flex gap-3 py-2 mr-2",
                            //         button {
                            //             r#type: "button",
                            //             class: "cursor-pointer inline-flex items-center gap-x-1.5 rounded-md p-2 text-xs font-medium bg-sky-400 hover:bg-sky-500 text-white",
                            //             onclick: handle_select_all,
                            //             "เลือกทุกคน"
                            //         }
                            //     }
                            // }
                        }

                        // div {
                        //     div { class: "mx-auto flex size-12 items-center justify-center rounded-full bg-green-100",
                        //         svg {
                        //             "stroke-width": "1.5",
                        //             stroke: "currentColor",
                        //             "aria-hidden": "true",
                        //             "viewBox": "0 0 24 24",
                        //             "data-slot": "icon",
                        //             fill: "none",
                        //             class: "size-6 text-green-600",
                        //             path {
                        //                 "stroke-linecap": "round",
                        //                 d: "m4.5 12.75 6 6 9-13.5",
                        //                 "stroke-linejoin": "round",
                        //             }
                        //         }
                        //     }
                        //     div { class: "mt-3 text-center sm:mt-5",
                        //         h3 {
                        //             class: "text-base font-semibold text-gray-900",
                        //             id: "modal-title",
                        //             "Payment successful"
                        //         }
                        //         div { class: "mt-2",
                        //             p { class: "text-sm text-gray-500",
                        //                 "Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequatur amet labore."
                        //             }
                        //         }
                        //     }
                        // }

                        div { class: "mt-5 sm:mt-6",
                            button {
                                class: "inline-flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                                r#type: "button",
                                onclick: handle_apply_order,
                                "Apply and close"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn exclude_member(members: &Members, name: &str) -> Members {
    let tmp = members.clone();
    tmp.iter()
        .filter(|&m| m.name != name)
        .map(|m| m.clone())
        .collect()
}
