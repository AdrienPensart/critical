use async_trait::async_trait;
use edgedb_derive::Queryable;
use edgedb_protocol::common::Cardinality as Cd;
use edgedb_protocol::value::Value as EValue;
use edgedb_tokio::Client;
use indexmap::indexmap;
use uuid::Uuid;

use crate::errors::CriticalErrorKind;
use crate::helpers::edge_object_from_pairs;
use crate::queries::UPSERT_QUERY;

pub mod flac_file;
pub mod mp3_file;

static RATINGS: &[f64] = &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];

#[derive(Queryable)]
pub struct MusicOutput {
    pub id: Uuid,
    pub name: String,
}

#[async_trait]
pub trait Music {
    fn path(&self) -> &str;
    fn folder(&self) -> &str;
    fn length(&self) -> i64;
    fn artist(&self) -> &str;
    fn album(&self) -> &str;
    fn title(&self) -> &str;
    fn genre(&self) -> &str;
    fn track(&self) -> i64;
    fn rating(&self) -> Result<f64, CriticalErrorKind>;
    fn keywords(&self) -> Vec<String>;
    fn links(&self) -> Vec<String>;
    async fn size(&self) -> Result<u64, CriticalErrorKind> {
        match async_fs::metadata(self.path()).await {
            Ok(metadata) => Ok(metadata.len()),
            Err(e) => return Err(CriticalErrorKind::IOError(e)),
        }
    }
    async fn upsert(
        &self,
        conn: &Client,
        username: &str,
        ipv4: &str,
    ) -> Result<MusicOutput, CriticalErrorKind> {
        let folder = self.folder().to_string();
        let rating = self.rating()?;
        let size = self.size().await?;

        let pairs = indexmap! {
            "artist" => (Some(EValue::Str(self.artist().to_string())), Cd::One),
            "album" => (Some(EValue::Str(self.album().to_string())), Cd::One),
            "genre" => (Some(EValue::Str(self.genre().to_string())), Cd::One),
            "keywords" => (Some(EValue::Array(self.keywords().into_iter().map(EValue::Str).collect())), Cd::One),
            "folder" => (Some(EValue::Str(folder)), Cd::One),
            "username" => (Some(EValue::Str(username.to_string())), Cd::One),
            "ipv4" => (Some(EValue::Str(ipv4.to_string())), Cd::One),
            "title" => (Some(EValue::Str(self.title().to_string())), Cd::One),
            "size" => (Some(EValue::Int64(size as i64)), Cd::One),
            "length" => (Some(EValue::Int64(self.length())), Cd::One),
            "track" => (Some(EValue::Int64(self.track())), Cd::One),
            "rating" => (Some(EValue::Float64(rating)), Cd::One),
            "path" => (Some(EValue::Str(self.path().to_string())), Cd::One),
        };
        let args = edge_object_from_pairs(pairs);
        let music_output: Result<MusicOutput, _> =
            conn.query_required_single(UPSERT_QUERY, &args).await;
        Ok(music_output?)
    }
}

impl std::fmt::Debug for dyn Music + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

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

impl std::fmt::Display for dyn Music + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
