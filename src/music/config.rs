use std::path::PathBuf;

use indradb::{Database, MemoryDatastore};

use super::errors::CriticalErrorKind;

pub struct Config {
    pub dsn: String,
    pub gel: gel_tokio::Client,
    pub dry: bool,
    pub datastore: PathBuf,
    pub indradb: Database<MemoryDatastore>,
    pub no_gel: bool,
    pub no_indradb: bool,
    pub retries: u16,
}

impl Config {
    pub fn new(
        dsn: String,
        datastore: String,
        dry: bool,
        no_gel: bool,
        no_indradb: bool,
    ) -> Result<Self, CriticalErrorKind> {
        let config = gel_tokio::Builder::new()
            .dsn(&dsn)
            // .client_security(edgedb_tokio::ClientSecurity::InsecureDevMode)
            .build()?;
        let gel = gel_tokio::Client::new(&config);

        let datastore: PathBuf = datastore.into();
        let indradb = if no_indradb {
            indradb::MemoryDatastore::new_db()
        } else if datastore.exists() {
            indradb::MemoryDatastore::read_msgpack_db(datastore.clone())?
        } else {
            indradb::MemoryDatastore::create_msgpack_db(datastore.clone())
        };

        Ok(Self {
            dry,
            dsn,
            gel,
            datastore,
            indradb,
            no_gel,
            no_indradb,
            retries: 0,
        })
    }
}
