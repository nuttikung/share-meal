use crate::state::member::Member;

pub fn is_member_exist(members: &Vec<Member>, new_member: &str) -> bool {
    let mut is_exist: bool = false;

    for member in members {
        if member.name.to_string() == new_member {
            is_exist = true;
            break;
        }
    }

    return is_exist;
}
