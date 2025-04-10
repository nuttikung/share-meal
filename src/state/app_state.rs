use super::{
    member::Members,
    order::{Order, Orders},
};

#[derive(Clone)]
pub struct AppState {
    pub view: String,
    pub orders: Orders,
    pub members: Members,
    pub seleted_order: Option<Order>,
}
