use super::{cache::UpsertCache, config::Config, errors::CriticalErrorKind};

#[async_trait::async_trait]
pub trait Vertex {
    async fn upsert_gel(
        &self,
        config: &Config,
        cache: &mut UpsertCache,
    ) -> Result<uuid::Uuid, CriticalErrorKind>;
}
