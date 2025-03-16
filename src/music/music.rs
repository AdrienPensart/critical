use std::hash::{DefaultHasher, Hash, Hasher};

use indradb::{
    BulkInsertItem, Database, Edge, Identifier, MemoryDatastore, Query, QueryOutputValue, Vertex,
    VertexWithPropertyValueQuery, ijson,
};
use serde::Serialize;

use super::cache::UpsertCache;
use super::config::Config;
use super::errors::CriticalErrorKind;
use super::ratings::Rating;

#[derive(Serialize)]
pub struct Music {
    pub title: String,
    pub path: String,
    pub size: i64,
    pub length: i64,
    pub track: i64,
    pub rating: Rating,

    pub keywords_indradb: Vec<uuid::Uuid>,
    pub keywords_gel: Vec<uuid::Uuid>,

    pub folder_indradb: uuid::Uuid,
    pub folder_gel: uuid::Uuid,

    pub artist_indradb: uuid::Uuid,
    pub artist_gel: uuid::Uuid,

    pub album_indradb: uuid::Uuid,
    pub album_gel: uuid::Uuid,

    pub genre_indradb: uuid::Uuid,
    pub genre_gel: uuid::Uuid,
}

impl Hash for Music {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.artist_indradb.hash(state);
        self.album_indradb.hash(state);
        self.genre_indradb.hash(state);
    }
}

const INDEX: &str = "music-unique-constraint";

#[async_trait::async_trait]
impl super::vertex::Vertex for Music {
    fn index_indradb(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind> {
        let unique_constraint = Identifier::new(INDEX)?;
        Ok(db.index_property(unique_constraint)?)
    }

    fn upsert_indradb(&self, config: &Config) -> Result<uuid::Uuid, CriticalErrorKind> {
        let id = Identifier::new("music")?;
        let title = Identifier::new("music-title")?;
        let album = Identifier::new("music-album")?;
        let artist = Identifier::new("music-artist")?;
        let genre = Identifier::new("music-genre")?;
        let length = Identifier::new("music-length")?;
        let track = Identifier::new("music-track")?;
        let size = Identifier::new("music-size")?;
        let rating = Identifier::new("music-rating")?;
        let keyword = Identifier::new("music-keyword")?;
        let unique_constraint = Identifier::new(INDEX)?;

        let vertex = Vertex::new(id);
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash_value = hasher.finish();

        let music_artist = Edge::new(vertex.id, artist, self.artist_indradb);
        let music_album = Edge::new(vertex.id, album, self.album_indradb);
        let music_genre = Edge::new(vertex.id, genre, self.genre_indradb);

        let results = config.indradb.get(Query::VertexWithPropertyValue(
            VertexWithPropertyValueQuery::new(unique_constraint, ijson!(hash_value)),
        ))?;

        let vertex_id = if let QueryOutputValue::Vertices(vertices) = &results[0] {
            if vertices.len() == 1 {
                vertices[0].id
            } else {
                let vertex_id = vertex.id;
                let mut items = vec![
                    BulkInsertItem::Vertex(vertex),
                    BulkInsertItem::VertexProperty(vertex_id, length, ijson!(self.length)),
                    BulkInsertItem::VertexProperty(vertex_id, track, ijson!(self.track)),
                    BulkInsertItem::VertexProperty(vertex_id, rating, ijson!(self.rating)),
                    BulkInsertItem::VertexProperty(vertex_id, size, ijson!(self.size)),
                    BulkInsertItem::Edge(music_artist),
                    BulkInsertItem::Edge(music_album),
                    BulkInsertItem::Edge(music_genre),
                    BulkInsertItem::VertexProperty(vertex_id, title, ijson!(self.title)),
                ];
                for keyword_id in &self.keywords_indradb {
                    items.push(BulkInsertItem::Edge(Edge::new(
                        vertex_id,
                        keyword,
                        *keyword_id,
                    )));
                }
                config.indradb.bulk_insert(items)?;
                vertex_id
            }
        } else {
            unreachable!();
        };
        Ok(vertex_id)
    }

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
            };
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
