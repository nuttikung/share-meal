use dioxus::prelude::*;

#[component]
pub fn OrderModalEdit() -> Element {
    rsx!(
        details {
            class: "open",
            summary {
                class: "bg-blue-300 rounded-2xl block px-6 py-4 text-xl hover:bg-blue-500 hover:duration-300 hover:text-white cursor-pointer",
                "Click me"
            }
            div {
                class: "bg-white rounded-2xl w-[50%] h-[40%] my-auto mx-auto absolute inset-0 text-gray-600 p-4 py-8",
                span {
                    "onclick": "document.querySelector('details').removeAttribute('open')",
                    class: "bg-gray-400 rounded-full flex items-center justify-center text-3xl font-bold uppercase absolute -right-4 -top-4 w-10 h-10 text-white hover:bg-blue-200 hover:duration-200 cursor-pointer",
                    "X"
                }
                h1 {
                    class: "text-2xl text-center text-blue-300 font-bold",
                    "This is a popup."
                }
                p {
                    class: "text-gray-600 text-base my-3 text-center",
                    "Lorem ipsum dolor sit amet consectetur adipisicing elit. Cum enim sint accusamus dolor, architectoomnis laborum tempora vitae, eveniet corporis qui officiis accusantium eius, esse corrupti possimusrepudiandae! Quod esse, sed laudantium nobis tenetur dolores impedit quaerat, ipsa molestias eaque."
                }
            }
        }
    )
}
