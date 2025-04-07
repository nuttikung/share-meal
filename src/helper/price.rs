// TODO: need check round up
pub fn round_up_float_to_one_precision(num: f64) -> String {
    let fmt_value = format!("{:.1}", num);
    if fmt_value == String::from("-0.0") {
        return String::from("0.0");
    }
    return fmt_value;
}

pub fn round_up_float_to_two_precision(num: f64) -> String {
    let fmt_value = format!("{:.2}", num);
    if fmt_value == String::from("-0.00") {
        return String::from("0.00");
    }
    return fmt_value;
}
