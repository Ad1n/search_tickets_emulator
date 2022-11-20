use md5::Digest;
use serde::{Deserialize, Serialize};
use crate::lmdb_repo::LMDB;
use crate::ticket::{ SimpleTicket, TicketDigest};

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequest {
    pub tickets: Vec<SimpleTicket>,
}

impl BatchRequest {
    pub fn insert_into_database(&self) -> Result<u32, &'static str> {
        let mut suceeded_inserts: u32 = 0;

        log::info!("Starting to insert: {}", self.tickets.len());

        // let Digest(composed_md5_as_str): md5::Digest = self.tickets[0].compose();
        // let res = LMDB.put_data(&Digest(composed_md5_as_str), &self.tickets[0]);
        // suceeded_inserts += 1;

        for ticket in self.tickets.iter() {
            let Digest(composed_md5_as_str): md5::Digest = ticket.compose();
            match LMDB.put_data(&Digest(composed_md5_as_str), &ticket) {
                Ok(_) => { suceeded_inserts += 1; },
                Err(err) => panic!("{}", err),
            }
        }

        // TODO: use object other than vector cause vector has no copy trait
        // let _ = self.tickets.clone().into_iter().map(|ticket| {
        //     log::info!("{}", ticket.id.clone());
        //     let Digest(composed_md5_as_str): md5::Digest = ticket.compose();
        //     match LMDB.put_data(&Digest(composed_md5_as_str), &ticket) {
        //         Ok(_) => { suceeded_inserts += 1; },
        //         Err(err) => panic!("{}", err),
        //     }
        // });

        Ok(suceeded_inserts)
    }
}