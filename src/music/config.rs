use indradb::{Database, MemoryDatastore};

pub struct Config {
    pub gel: gel_tokio::Client,
    pub dry: bool,
    pub indradb: Database<MemoryDatastore>,
    pub no_gel: bool,
    pub no_indradb: bool,
    pub retries: u16,
}
