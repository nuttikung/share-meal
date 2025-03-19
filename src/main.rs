use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

// region :      --- Main Function
fn main() {
    dioxus::launch(App);
}
// end region :  --- Main Function

// region :      --- AppState
type Member = String;
type Members = Vec<String>;
type Orders = Vec<Order>;

#[derive(Clone)]
struct Order {
    id: i32,
    title: String,
    price: f32,
    members: Members,
}

#[derive(Clone)]
struct AppState {
    view: String,
    orders: Orders,
    members: Members,
}
// end region :  --- AppState

#[component]
fn App() -> Element {
    use_context_provider(|| {
        Signal::new(AppState {
            view: String::from("orders"),
            orders: vec![],
            members: vec![String::from("test")],
        })
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Overview {}
    }
}

#[component]
pub fn Overview() -> Element {
    let mut context = use_context::<Signal<AppState>>();

    // region :      --- Handle Order Click
    let handleOrderClick = move |_| {
        context.write().view = String::from("orders");
    };

    let handleMemberClick = move |_| {
        context.write().view = String::from("members");
    };
    // end region :  --- Handle Order Click

    rsx! {
        main {
            div {
                id: "overview",
                div {
                    class: "overvie-item",
                    "จำนวนคน"
                    div {
                        id: "overvie-item-total-person",
                        class: "overvie-item-total",
                        "0"
                    }
                }

                div {
                    class: "overvie-item",
                    "ราคารวม"
                    div {
                        id: "overvie-item-total-price",
                        class: "overvie-item-total",
                        "0"
                    }
                }
            }
            div {
                id: "tab",
                button {
                    id: "orders",
                    onclick: handleOrderClick,
                    "รายการ"
                }
                button {
                    id: "members",
                    onclick: handleMemberClick,
                    "คนจ่าย"
                }
            }

            if context().view == "orders" {
                OrderList {}
            } else {
                MemberList {}
                div {
                    MemberInput {}
                }
            }

        }
    }
}

#[component]
pub fn OrderList() -> Element {
    rsx! {
        div {
            id: "overview",
            "this is order list view"
        }
    }
}

#[component]
pub fn MemberList() -> Element {
    let  context = use_context::<Signal<AppState>>();
    rsx! {
        div {
            id: "overview",
            for person in &context.read().members {
                div {
                    "{person}"
                }
            }
        }
    }
}

#[component]
pub fn MemberInput() -> Element {
    rsx! {
        input {
            class: "input",
            placeholder: "ระบุชื่อ",
            value: "",
            autofocus: "true",
            // oninput: move |evt| draft.set(evt.value()),
            // onkeydown
        }
    }
}
