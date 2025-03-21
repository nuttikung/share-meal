use super::{member::Members, order::Orders};

#[derive(Clone)]
pub struct AppState {
    pub view: String,
    pub orders: Orders,
    pub members: Members,
}
