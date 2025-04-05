use super::cache::UpsertCache;
use super::config::Config;
use super::errors::CriticalErrorKind;
use super::ratings::Rating;

pub struct Music {
    pub title: String,
    pub path: String,
    pub size: i64,
    pub length: i64,
    pub track: i64,
    pub rating: Rating,
    pub keywords_gel: Vec<uuid::Uuid>,
    pub folder_gel: uuid::Uuid,
    pub artist_gel: uuid::Uuid,
    pub album_gel: uuid::Uuid,
    pub genre_gel: uuid::Uuid,
}

#[async_trait::async_trait]
impl super::vertex::Vertex for Music {
    async fn upsert_gel(
        &self,
        config: &Config,
        cache: &mut UpsertCache,
    ) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_gel {
            return Ok(uuid::Uuid::new_v4());
        }

        let mut music_id: Option<uuid::Uuid> = None;
        for _ in 0..config.retries {
            let rating: f64 = self.rating.into();
            let music_result = Box::pin(config.gel.query_required_single(
                UPSERT_MUSIC,
                &(
                    &self.title,
                    &self.album_gel,
                    &self.genre_gel,
                    &self.size,
                    &self.length,
                    self.keywords_gel.clone(),
                    &self.track,
                    &rating,
                    &self.folder_gel,
                    &self.path,
                ),
            ))
            .await;
            match music_result {
                Err(e) => {
                    if e.kind_name() != "TransactionSerializationError" {
                        return Err(CriticalErrorKind::GelErrorWithObject {
                            error: e,
                            object: self.path.clone(),
                        });
                    }
                    // load_music_files_bar.println(format!("retrying upsert music {}", music.title()));
                    cache.errors += 1;
                }
                Ok(id) => {
                    music_id = Some(id);
                    break;
                }
            }
        }
        let Some(music_id) = music_id else {
            return Err(CriticalErrorKind::UpsertError {
                path: self.path.clone(),
                object: self.path.clone(),
            });
        };
        Ok(music_id)
    }
}

const UPSERT_MUSIC: &str = "
select upsert_music(
    title := <str>$0,
    size := <Size>$3,
    length := <Length>$4,
    genre := <Genre>$2,
    album := <Album>$1,
    keywords := <array<uuid>>$5,
    track := <optional Track>$6,
    rating := <Rating>$7,
    folder := <Folder>$8,
    path := <str>$9
).id";

pub const MUSIC_FIELDS: &str = r"
name,
artist_name := .artist.name,
album_name := .album.name,
genre_name := .genre.name,
length,
human_duration,
size,
human_size,
track,
rating,
keywords_names := (select .keywords.name),
folders: {
    name,
    username,
    ipv4,
    path := @path
}
";
