pub fn is_member_exist(members: &Vec<String>, new_member: &str) -> bool {
    let mut is_exist: bool = false;

    for member in members {
        if member == new_member {
            is_exist = true;
            break;
        }
    }

    return is_exist;
}
