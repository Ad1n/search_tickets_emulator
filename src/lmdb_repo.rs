use crate::ticket::SimpleTicket;
use heed::types::Str;
use heed::{Database, EnvOpenOptions};
use std::fs;
use std::path::Path;

pub struct LmdbRepo {
    pub db: Database<Str, Str>,
    pub env: heed::Env,
    pub env_path: std::path::PathBuf,
    // collection: Vec<SimpleTicket>
}

// https://users.rust-lang.org/t/one-global-variable-for-mysql-connection/49063
lazy_static! {
    pub static ref LMDB: LmdbRepo = LmdbRepo::new();
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
        let db_obj: Database<Str, Str> = env.create_database(Some("test")).unwrap();

        LmdbRepo {
            db: db_obj,
            env,
            env_path,
        }
    }

    pub fn put_data<'b>(
        &self,
        key: &md5::Digest,
        value: &'b SimpleTicket,
    ) -> Result<&'b SimpleTicket, &'static str> {
        let mut wtxn = LMDB.env.write_txn().unwrap();
        let as_str_key: &str = &format!("{:x}", key)[..];
        let borrowed_value: String = serde_json::to_string(&value).unwrap();
        let serialized_value = borrowed_value.as_str();
        LMDB.db
            .put(&mut wtxn, as_str_key, serialized_value)
            .expect("Write to database failed");
        match wtxn.commit() {
            Ok(_) => Ok(value),
            Err(_) => Err("Transaction failed to commit"),
        }
    }
}
