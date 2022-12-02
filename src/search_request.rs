use crate::ticket_solution::TicketSolution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    departure_code: String,
    arrival_code: String,
    departure_date: String, // ISO 8601 date
    limit: i32,
}

// impl SearchRequest {
//     pub fn compose_tickets_v1(&self) -> Result<Vec<TicketSolution>, &'static str> {}
// }
