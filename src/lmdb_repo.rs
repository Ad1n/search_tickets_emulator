use std::fs;
use std::path::Path;
use heed::{Database, EnvOpenOptions, Error};
use serde::de::Unexpected::Option;
use crate::ticket;
// use crate::ticket;
use crate::ticket::SimpleTicket;

pub struct LmdbRepo {
    db: Database<md5::Digest, SimpleTicket>,
    env: heed::Env,
    env_path: std::path::PathBuf,
    // collection: Vec<SimpleTicket>
}

// https://users.rust-lang.org/t/one-global-variable-for-mysql-connection/49063
lazy_static! {
    static ref LMDB: LmdbRepo = LmdbRepo::new();
}

impl LmdbRepo {
    pub fn new() -> LmdbRepo {
        // Init LMDB
        let env_path = Path::new("target").join("test-database.mdb");
        let _ = fs::remove_dir_all(&env_path);
        fs::create_dir_all(&env_path).unwrap();
        let env = EnvOpenOptions::new()
            .map_size(10 * 1024 * 1024) // 10MB
            .max_dbs(3)
            .open(&env_path)
            .unwrap();
        let db_obj: Database<md5::Digest, ticket::SimpleTicket> =
            env.create_database(Some("test")).unwrap();

        LmdbRepo {
            db: db_obj,
            env,
            env_path
        }
    }

    pub fn put_data<'b>(key: &md5::Digest, value: &'b SimpleTicket) -> Result<&'b SimpleTicket, &'static str> {
        let mut wtxn = LMDB.env.write_txn().unwrap();
        LMDB.db.put(&mut wtxn, &key, &value).expect("Write to database failed");
        match wtxn.commit() {
            Ok(_) => Ok(value),
            Err(_) => Err("invalid header length"),
        }
    }
}