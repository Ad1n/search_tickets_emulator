use crate::ticket_solution::TicketSolution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub(crate) solutions: Vec<TicketSolution>,
}
