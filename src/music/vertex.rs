use indradb::{Database, MemoryDatastore};

use super::{cache::UpsertCache, config::Config, errors::CriticalErrorKind};

#[async_trait::async_trait]
pub trait Vertex {
    fn index_indradb(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind>;
    fn upsert_indradb(&self, config: &Config) -> Result<uuid::Uuid, CriticalErrorKind>;
    async fn upsert_gel(
        &self,
        config: &Config,
        cache: &mut UpsertCache,
    ) -> Result<uuid::Uuid, CriticalErrorKind>;
}
