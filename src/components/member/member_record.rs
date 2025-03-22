use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct MemberRecordProps {
    member: String,
}

#[component]
pub fn MemberRecord(props:MemberRecordProps) -> Element {
    // find the price for one person
    rsx!(
        div {
            class: "member-overview",
            id: "overview",

            div {
                class: "member-list",
                "{props.member}"
            }

            div {
                class: "price-payment",
                "xxx"
            }

            div {
                class: "price-payment",
                "จ่าย"
            }
        }
    )
}
