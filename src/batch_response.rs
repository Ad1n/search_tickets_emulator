use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResponse {
    count: u32,
}

impl BatchResponse {
    pub fn new(count: u32) -> BatchResponse {
        BatchResponse { count }
    }
}