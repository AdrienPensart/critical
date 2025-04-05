use super::{cache::UpsertCache, config::Config, errors::CriticalErrorKind};

pub struct Album {
    pub name: String,
    pub artist_gel: uuid::Uuid,
}

#[async_trait::async_trait]
impl super::vertex::Vertex for Album {
    async fn upsert_gel(
        &self,
        config: &Config,
        cache: &mut UpsertCache,
    ) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_gel {
            return Ok(uuid::Uuid::new_v4());
        }

        if cache.albums[&self.artist_gel].contains_key(&self.name) {
            return Ok(cache.albums[&self.artist_gel][&self.name]);
        }
        let mut album_id: Option<uuid::Uuid> = None;
        for _ in 0..config.retries {
            let album_result = Box::pin(
                config
                    .gel
                    .query_required_single(UPSERT_ALBUM, &(&self.name, &self.artist_gel)),
            )
            .await;
            match album_result {
                Err(e) => {
                    if e.kind_name() != "TransactionSerializationError" {
                        return Err(CriticalErrorKind::GelErrorWithObject {
                            error: e,
                            object: self.name.clone(),
                        });
                    }
                    // load_music_files_bar.println(format!("retrying upsert album {}", music.album()));
                    cache.errors += 1;
                }
                Ok(id) => {
                    album_id = Some(id);
                    break;
                }
            }
        }
        let Some(album_id) = album_id else {
            return Err(CriticalErrorKind::UpsertError {
                path: self.name.clone(),
                object: self.name.clone(),
            });
        };
        if let Some(albums) = cache.albums.get_mut(&self.artist_gel) {
            albums.insert(self.name.to_string(), album_id);
        }
        Ok(album_id)
    }
}

const UPSERT_ALBUM: &str = "
select upsert_album(
    artist := <Artist>$1,
    album := <str>$0
).id";
