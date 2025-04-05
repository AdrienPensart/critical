use super::{cache::UpsertCache, config::Config, errors::CriticalErrorKind};

pub struct Keyword {
    pub name: String,
}

#[async_trait::async_trait]
impl super::vertex::Vertex for Keyword {
    async fn upsert_gel(
        &self,
        config: &Config,
        cache: &mut UpsertCache,
    ) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_gel {
            return Ok(uuid::Uuid::new_v4());
        }

        if cache.keywords.contains_key(&self.name) {
            return Ok(cache.keywords[&self.name]);
        }
        let mut keyword_id: Option<uuid::Uuid> = None;
        for _ in 0..config.retries {
            let keyword_folder = Box::pin(
                config
                    .gel
                    .query_required_single(UPSERT_KEYWORD, &(&self.name,)),
            )
            .await;
            match keyword_folder {
                Err(e) => {
                    if e.kind_name() != "TransactionSerializationError" {
                        return Err(CriticalErrorKind::GelErrorWithObject {
                            error: e,
                            object: self.name.clone(),
                        });
                    }
                    // load_music_files_bar.println(format!("retrying upsert keyword {}", self.name));
                    cache.errors += 1;
                }
                Ok(id) => {
                    keyword_id = Some(id);
                    break;
                }
            }
        }
        let Some(keyword_id) = keyword_id else {
            return Err(CriticalErrorKind::UpsertError {
                path: self.name.clone(),
                object: self.name.clone(),
            });
        };
        cache.keywords.insert(self.name.clone(), keyword_id);
        Ok(keyword_id)
    }
}

const UPSERT_KEYWORD: &str = "
select upsert_keyword(
    keyword := <str>$0
).id";
