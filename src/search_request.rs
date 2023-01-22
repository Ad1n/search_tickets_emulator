use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub departure_code: String,
    pub arrival_code: String,
    departure_date: String, // ISO 8601 date
    limit: i32,
}
