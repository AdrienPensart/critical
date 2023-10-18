use std::fmt;
use edgedb_tokio::Client;
use edgedb_protocol::value::Value as EValue;
use edgedb_protocol::common::Cardinality as Cd;
use edgedb_derive::Queryable;
use uuid::Uuid;
use indexmap::indexmap;
use async_trait::async_trait;

use crate::errors::CriticalErrorKind;
use crate::helpers::edge_object_from_pairs;
use crate::queries::UPSERT_QUERY;

pub mod flac_file;
pub mod mp3_file;

static RATINGS: &[f64] = &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];

#[derive(Queryable)]
pub struct MusicOutput {
    pub id: Uuid,
    pub name: String
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
    fn size(&self) -> u64;
    fn track(&self) -> i64;
    fn rating(&self) -> Result<f64, CriticalErrorKind>;
    fn keywords(&self) -> Vec<String>;
    fn links(&self) -> Vec<String>;
    async fn upsert(&self, conn: &Client, username: &str, ipv4: &str) -> Result<MusicOutput, CriticalErrorKind> {
        let folder = self.folder().to_string();
        let rating = self.rating()?;

        let pairs = indexmap! {
            "artist" => (Some(EValue::Str(self.artist().to_string())), Cd::One),
            "album" => (Some(EValue::Str(self.album().to_string())), Cd::One),
            "genre" => (Some(EValue::Str(self.genre().to_string())), Cd::One),
            "keywords" => (Some(EValue::Array(self.keywords().into_iter().map(EValue::Str).collect())), Cd::One),
            "folder" => (Some(EValue::Str(folder)), Cd::One),
            "user" => (Some(EValue::Str(username.to_string())), Cd::One),
            "ipv4" => (Some(EValue::Str(ipv4.to_string())), Cd::One),
            "title" => (Some(EValue::Str(self.title().to_string())), Cd::One),
            "size" => (Some(EValue::Int64(self.size() as i64)), Cd::One),
            "length" => (Some(EValue::Int64(self.length())), Cd::One),
            "track" => (Some(EValue::Int64(self.track())), Cd::One),
            "rating" => (Some(EValue::Float64(rating)), Cd::One),
            "path" => (Some(EValue::Str(self.path().to_string())), Cd::One),
        };
        let args = edge_object_from_pairs(pairs);
        let music_output: Result<MusicOutput, _> = conn.query_required_single(
            UPSERT_QUERY,
            &args,
        ).await;
        Ok(music_output?)
    }
}

impl fmt::Debug for dyn Music {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Music")
            .field("path", &self.path())
            .field("folder", &self.folder())
            .field("size", &self.size())
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

impl fmt::Display for dyn Music {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path())
    }
}
