// pub type Member = String;

#[derive(PartialEq, Clone, Debug)]
pub struct Member {
    pub name: String,
    pub paid: bool,
}

pub type Members = Vec<Member>;
