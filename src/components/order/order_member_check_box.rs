use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;
use hextool::{Convert, Hex};

#[derive(PartialEq, Props, Clone, Debug)]
pub struct OrderMemberCheckBoxProps {
    name: String,
    selected: bool,
    onselect: Callback<bool>,
}

#[component]
pub fn OrderMemberCheckBox(props: OrderMemberCheckBoxProps) -> Element {
    let OrderMemberCheckBoxProps {
        name,
        selected,
        onselect,
    } = props;

    // Find function convert name to hex or hue color
    let name_color = Hex::convert(&name, false, false);
    // tracing::debug!("{}",name_color);

    // region :      --- Handle Toggle Check
    let handle_check_member = move |_| {
        onselect.call(!props.selected);
    };
    // end region :  --- Handle Toggle Check

    rsx!(
        div {
            class: "relative inline-flex gap-3 py-2 mr-2",
            span {
                class: "inline-flex items-center gap-x-1.5 rounded-md p-2 text-xs font-medium",
                class: if selected { "bg-[#{name_color}] text-[#{name_color}]" } else { "bg-gray-100 text-gray-600" },
                onclick: handle_check_member,
                if selected {
                    MaterialIcon {
                        size: 10,
                        name: "check"
                    }
                } else {
                    MaterialIcon {
                        size: 10,
                        name: "add"
                    }
                }
                "{name}"
            }
        }
    )
}
