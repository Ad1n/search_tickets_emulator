use serde::{Deserialize, Serialize};
use crate::ticket::SimpleTicket;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequest {
    pub tickets: Vec<SimpleTicket>,
}