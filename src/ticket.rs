use md5;
use serde::{Deserialize, Serialize};
use sha1;

pub trait TicketDigest {
    fn compose(&self) -> md5::Digest;
}

// id - String, 32 символа, уникальный идентификатор
// departure_code - String, iata код аэропорта
// arrival_code - String, iata код аэропорта
// departure_time - Integer, unix timestamp
// arrival_time - Integer, unix timestamp
// price - Integer, цена в рублях
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleTicket {
    pub id: String,
    pub departure_code: String,
    pub arrival_code: String,
    pub departure_time: u32,
    pub arrival_time: u32,
    price: u32,
}

impl Default for SimpleTicket {
    fn default() -> SimpleTicket {
        SimpleTicket {
            id: format!("{:?}", sha1::Sha1Core::default()),
            departure_code: "".to_string(),
            arrival_code: "".to_string(),
            departure_time: 0,
            arrival_time: 0,
            price: 0,
        }
    }
}

impl TicketDigest for SimpleTicket {
    fn compose(&self) -> md5::Digest {
        let composed_string: String = vec![
            self.departure_code.to_string(),
            self.arrival_code.to_string(),
            self.departure_time.to_string(),
            self.arrival_time.to_string(),
        ]
        .join("_");

        md5::compute(composed_string.as_bytes())
    }
}

pub struct SmartTicket {
    //TODO:()
}
