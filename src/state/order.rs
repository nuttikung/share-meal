use super::member::Members;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub title: String,
    pub price: f64,
    pub members: Members,
}

pub type Orders = Vec<Order>;
