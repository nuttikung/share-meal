use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone, Debug)]
pub struct OrderMemberCheckBoxProps {
    name: String,
    selected: bool,
    onselect: Callback<Event<FormData>>,
}

#[component]
pub fn OrderMemberCheckBox(props: OrderMemberCheckBoxProps) -> Element {
    // region :      --- Handle Check Box Click
    let handle_check_member = move |event:Event<FormData>| {
        props.onselect.call(event);
    };
    // end region :  --- Handle Check Box Click

    rsx!(
        div {
            class: "relative flex gap-3 py-4",
            div {
                class: "min-w-0 flex-1 text-sm/6",
                label {
                    class: "font-medium text-gray-900 select-none",
                    for: "person-1",
                    "{props.name}"
                }
            }
            div {
                class: "flex h-6 shrink-0 items-center",
                div {
                    class: "group grid size-4 grid-cols-1",
                    input {
                        name: "person-1",
                        type: "checkbox",
                        checked: "false",
                        class: "col-start-1 row-start-1 appearance-none rounded-sm border border-gray-300 bg-white checked:border-blue-600 checked:bg-blue-600 indeterminate:border-blue-600 indeterminate:bg-blue-600 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:border-gray-300 disabled:bg-gray-100 disabled:checked:bg-gray-100 forced-colors:appearance-auto",
                        id: "person-1",
                        onchange: handle_check_member,
                        oninput: handle_check_member
                    }
                    svg {
                        "viewBox": "0 0 14 14",
                        fill: "none",
                        class: "pointer-events-none col-start-1 row-start-1 size-3.5 self-center justify-self-center stroke-white group-has-disabled:stroke-gray-950/25",
                        path {
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            d: "M3 8L6 11L11 3.5",
                            class: "opacity-0 group-has-checked:opacity-100",
                        }
                        path {
                            "stroke-linecap": "round",
                            d: "M3 7H11",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            class: "opacity-0 group-has-indeterminate:opacity-100",
                        }
                    }
                }
            }
        }
    )
}
