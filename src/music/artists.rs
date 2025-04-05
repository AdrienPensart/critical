use std::collections::HashMap;

use super::{cache::UpsertCache, config::Config, errors::CriticalErrorKind};

pub const ARTISTS_QUERY: &str = r"
select Artist {
    name,
    rating,
    length,
    duration,
    size,
    all_keywords := array_agg(.keywords.name),
    all_genres := array_agg(.musics.genre.name),
    n_albums := count(.albums),
    n_musics := count(.musics)
}
order by .name
";

pub struct Artist {
    pub name: String,
}

#[async_trait::async_trait]
impl super::vertex::Vertex for Artist {
    async fn upsert_gel(
        &self,
        config: &Config,
        cache: &mut UpsertCache,
    ) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_gel {
            return Ok(uuid::Uuid::new_v4());
        }

        if cache.artists.contains_key(&self.name) {
            return Ok(cache.artists[&self.name]);
        }

        let mut artist_id: Option<uuid::Uuid> = None;
        for _ in 0..config.retries {
            let artist_result = Box::pin(
                config
                    .gel
                    .query_required_single(UPSERT_ARTIST, &(&self.name,)),
            )
            .await;
            match artist_result {
                Err(e) => {
                    if e.kind_name() != "TransactionSerializationError" {
                        return Err(CriticalErrorKind::GelErrorWithObject {
                            error: e,
                            object: self.name.clone(),
                        });
                    }
                    cache.errors += 1;
                }
                Ok(id) => {
                    artist_id = Some(id);
                    break;
                }
            }
        }
        let Some(artist_id) = artist_id else {
            return Err(CriticalErrorKind::UpsertError {
                path: self.name.clone(),
                object: self.name.clone(),
            });
        };
        cache.artists.insert(self.name.clone(), artist_id);
        {
            cache.albums.insert(artist_id, HashMap::new());
        }
        Ok(artist_id)
    }
    // println!("artist_id: {artist_id}");
}

const UPSERT_ARTIST: &str = "
select upsert_artist(
    artist := <str>$0
).id";
