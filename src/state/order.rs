use super::member::Members;

#[derive(Clone)]
pub struct Order {
    pub id: i32,
    pub title: String,
    pub price: f32,
    pub members: Members,
}

pub type Orders = Vec<Order>;
