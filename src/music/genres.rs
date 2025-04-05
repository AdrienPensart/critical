use super::{cache::UpsertCache, config::Config, errors::CriticalErrorKind};

pub struct Genre {
    pub name: String,
}

#[async_trait::async_trait]
impl super::vertex::Vertex for Genre {
    async fn upsert_gel(
        &self,
        config: &Config,
        cache: &mut UpsertCache,
    ) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_gel {
            return Ok(uuid::Uuid::new_v4());
        }

        if cache.genres.contains_key(&self.name) {
            return Ok(cache.genres[&self.name]);
        }

        let mut genre_id: Option<uuid::Uuid> = None;
        for _ in 0..config.retries {
            let genre_result = Box::pin(
                config
                    .gel
                    .query_required_single(UPSERT_GENRE, &(&self.name,)),
            )
            .await;
            match genre_result {
                Err(e) => {
                    if e.kind_name() != "TransactionSerializationError" {
                        return Err(CriticalErrorKind::GelErrorWithObject {
                            error: e,
                            object: self.name.clone(),
                        });
                    }
                    // load_music_files_bar.println(format!("retrying upsert genre {}", self.name));
                    cache.errors += 1;
                }
                Ok(id) => {
                    genre_id = Some(id);
                    break;
                }
            }
        }
        let Some(genre_id) = genre_id else {
            return Err(CriticalErrorKind::UpsertError {
                path: self.name.clone(),
                object: self.name.clone(),
            });
        };
        cache.genres.insert(self.name.clone(), genre_id);
        Ok(genre_id)
    }
}

const UPSERT_GENRE: &str = "
select upsert_genre(
    genre := <str>$0
).id";
