use dioxus::prelude::*;

const SUGGEST_MENUS: [&'static str; 24] = [
    "อาหาร",
    "บุฟเฟ่ต์",
    "ข้าวมันไก่",
    "ข้าวมันไก่ทอด",
    "ก๋วยเตี๋ยวเรือ (ชามเล็ก)",
    "ก๋วยเตี๋ยวเรือ (ชามใหญ่)",
    "เครื่องดื่ม",
    "น้ำอัดลม",
    "น้ำเปล่า",
    "น้ำผลไม้",
    "น้ำดื่มรีฟิล",
    "น้ำแข็ง",
    "ชาเขียว",
    "ชานม (ไม่มุก)",
    "ชานม (ไข่มุก)",
    "เบียร์ (โปร)",
    "เหล้า (โปร)",
    "เบียร์",
    "เหล้า",
    "เหล้าปั่น",
    "มิกเซอร์",
    "VAT",
    "Service Charge",
    "ส่วนลด (Discount)",
];

#[component]
pub fn OrderInput(name: String, oninput: EventHandler<FormEvent>) -> Element {
    rsx! {
        input {
            id: "order-name",
            class: "block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-blue-500 sm:text-sm/6",
            autocomplete: "off",
            r#type: "text",
            name: "order_name",
            list: "sugges-menu",
            value: "{name}",
            onchange: move |event| oninput.call(event),
        }
        datalist {
            id: "sugges-menu",
            for o in &SUGGEST_MENUS  {
                option {
                    key: "{o}",
                    "{o}"
                }
            }
        }
    }
}
