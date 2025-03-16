pub mod albums;
pub mod artists;
pub mod bests;
pub mod clean;
pub mod errors;
pub mod filter;
pub mod flac_file;
pub mod folders;
pub mod genres;
pub mod helpers;
pub mod keywords;
pub mod mp3_file;
pub mod music_input;
pub mod music_result;
pub mod playlist;
pub mod ratings;
pub mod remove;
pub mod scan;
pub mod search;
pub mod stats;

use std::hash::{DefaultHasher, Hash, Hasher};

use indradb::{
    ijson, BulkInsertItem, Database, Edge, Identifier, MemoryDatastore, Query, QueryOutputValue,
    Vertex, VertexWithPropertyValueQuery,
};

use errors::CriticalErrorKind;
use ratings::Rating;
use serde::Serialize;

#[async_trait::async_trait]
pub trait Music {
    fn path(&self) -> &str;
    fn folder(&self) -> &str;
    fn length(&self) -> i64;
    fn artist(&self) -> &str;
    fn album(&self) -> &str;
    fn title(&self) -> &str;
    fn genre(&self) -> &str;
    fn track(&self) -> i64;
    fn rating(&self) -> Result<Rating, CriticalErrorKind>;
    fn keywords(&self) -> Vec<String>;
    async fn size(&self) -> Result<u64, CriticalErrorKind> {
        Ok(async_fs::metadata(self.path()).await?.len())
    }
    fn links(&self) -> Vec<String> {
        vec![String::from(self.path())]
    }
}

impl std::fmt::Debug for dyn Music + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| std::fmt::Error)?;

        let size = rt.block_on(self.size());

        f.debug_struct("Music")
            .field("path", &self.path())
            .field("folder", &self.folder())
            .field("size", &size)
            .field("artist", &self.artist())
            .field("album", &self.album())
            .field("title", &self.title())
            .field("track", &self.track())
            .field("length", &self.length())
            .field("rating", &self.rating())
            .field("keywords", &self.keywords())
            .field("links", &self.links())
            .finish()
    }
}

impl std::fmt::Display for dyn Music {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}

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

#[derive(Serialize)]
pub struct MusicVertex {
    pub title: String,
    pub artist_id: uuid::Uuid,
    pub album_id: uuid::Uuid,
    pub genre_id: uuid::Uuid,
    pub length: i64,
    pub track: i64,
    pub rating: Rating,
    pub keywords: Vec<uuid::Uuid>,
}

impl Hash for MusicVertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.artist_id.hash(state);
        self.album_id.hash(state);
        self.genre_id.hash(state);
    }
}

const INDEX: &str = "music-unique-constraint";

impl MusicVertex {
    pub fn index(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind> {
        let unique_constraint = Identifier::new(INDEX)?;
        Ok(db.index_property(unique_constraint)?)
    }

    pub fn upsert(&self, db: &Database<MemoryDatastore>) -> Result<uuid::Uuid, CriticalErrorKind> {
        let id = Identifier::new("music")?;
        let title = Identifier::new("music-title")?;
        let album = Identifier::new("music-album")?;
        let artist = Identifier::new("music-artist")?;
        let genre = Identifier::new("music-genre")?;
        let length = Identifier::new("music-length")?;
        let track = Identifier::new("music-track")?;
        let rating = Identifier::new("music-rating")?;
        let keyword = Identifier::new("music-keyword")?;

        let unique_constraint = Identifier::new(INDEX)?;
        db.index_property(unique_constraint)?;

        let vertex = Vertex::new(id);
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash_value = hasher.finish();

        let music_artist = Edge::new(vertex.id, artist, self.artist_id);
        let music_album = Edge::new(vertex.id, album, self.album_id);
        let music_genre = Edge::new(vertex.id, genre, self.genre_id);

        let results = db.get(Query::VertexWithPropertyValue(
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
                    BulkInsertItem::Edge(music_artist),
                    BulkInsertItem::Edge(music_album),
                    BulkInsertItem::Edge(music_genre),
                    BulkInsertItem::VertexProperty(vertex_id, title, ijson!(self.title)),
                ];
                for keyword_id in &self.keywords {
                    items.push(BulkInsertItem::Edge(Edge::new(
                        vertex_id,
                        keyword,
                        *keyword_id,
                    )));
                }
                db.bulk_insert(items)?;
                vertex_id
            }
        } else {
            unreachable!();
        };
        Ok(vertex_id)
    }
}
