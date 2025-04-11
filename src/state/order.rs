use super::member::Members;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub title: String,
    pub price: f64,
    pub members: Members,
}

impl Order {
    /// Utility function to check if member name is in this order.
    ///
    /// `true` when name is included,
    ///
    /// `false` when name is not included
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let members = vec![{ name: "a", paid: false },{ name: "b", paid: false }]
    /// assert_eq!(has_self_in(&members, "a"), true);
    /// assert_eq!(has_self_in(&members, "c"), false);
    /// ```
    pub fn has_member_in(&self, name: &str) -> bool {
        let mut found: bool = false;
        for n in &self.members {
            if n.name == name {
                found = true;
                break;
            }
        }
        return found;
    }

    /// Utility function to calculate the price in the order per a member
    ///
    /// ```
    /// let members: Members = vec![{ name: "a", paid: false },{ name: "b", paid: false }]
    /// let order = Order { id: "1", title: "example", price: 300, members: members }
    /// assert_eq!(order.calculate_price_per_member(), 100);
    /// ```
    pub fn calculate_price_per_member(&self) -> f64 {
        if self.members.len() == 0 {
            return 0 as f64;
        }

        return &self.price / (self.members.len() as f64);
    }

    pub fn remove_member(&mut self, name: &str) {
        self.members.retain(|m| m.name != name);
    }

    /// Utility function to clear all members in this order
    pub fn clear_members(&mut self) {
        self.members = vec![];
    }
}

pub type Orders = Vec<Order>;
