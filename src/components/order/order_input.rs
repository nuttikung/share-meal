use dioxus::prelude::*;

#[component]
pub fn OrderInput(name: String, oninput: EventHandler<FormEvent>) -> Element {
    rsx! {
        input {
            id: "order-name",
            class: "block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-blue-500 sm:text-sm/6",
            autocomplete: "off",
            r#type: "text",
            name: "order_name",
            value: "{name}",
            onchange: move |event| oninput.call(event),
        }
    }
}
