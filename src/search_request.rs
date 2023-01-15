use crate::ticket_solution::TicketSolution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub departure_code: String,
    pub arrival_code: String,
    departure_date: String, // ISO 8601 date
    limit: i32,
}

// impl SearchRequest {
//     pub fn perform_search(&self) -> Result<Vec<TicketSolution>, &'static str> {
//
//     }
// }
