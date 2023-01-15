use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TicketSolution {
    pub(crate) ticket_ids: Vec<String>,
    pub(crate) price: i32,
}
