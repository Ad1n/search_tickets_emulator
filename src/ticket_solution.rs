use serde::{Deserialize, Serialize};
use crate::lmdb_repo::LMDB;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TicketSolution {
    pub(crate) ticket_ids: Vec<String>,
    pub(crate) price: u32,
}

impl TicketSolution {
    pub fn recount_price(&mut self) -> () {
        self.price = count_price(&self.ticket_ids)
    }
}

pub fn count_price(ticket_ids: &Vec<String>) -> u32 {
    ticket_ids.iter().map(|e| {
        LMDB.read_data(&e[..]).unwrap().price
    })
        .sum()
}
