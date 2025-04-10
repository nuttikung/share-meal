use dioxus::prelude::*;

#[component]
pub fn OrderPriceInput(
    value: f64,
    currency: Option<String>,
    placeholder: Option<String>,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let default_currency = currency.unwrap_or_else(|| String::from("฿"));
    let default_placeholder = placeholder.unwrap_or_else(|| String::from(""));

    rsx! {
        div {
            class: "shrink-0 text-base text-gray-500 select-none sm:text-sm/6",
            "{default_currency}"
        }
        input {
            id: "order-price",
            class: "block min-w-0 grow py-1.5 pr-3 pl-1 text-base text-gray-900 placeholder:text-gray-400 focus:outline-none sm:text-sm/6",
            placeholder: "{default_placeholder}",
            type: "number",
            name: "price",
            value: "{value}",
            aria_describedby: "price-currency",
            onchange: move |event| oninput.call(event),
        }
    }
}
