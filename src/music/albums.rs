use std::hash::{DefaultHasher, Hash, Hasher};

use indradb::{
    BulkInsertItem, Database, Edge, Identifier, MemoryDatastore, Query, QueryOutputValue, Vertex,
    VertexWithPropertyValueQuery, ijson,
};
use serde::Serialize;

use super::{cache::UpsertCache, config::Config, errors::CriticalErrorKind};

#[derive(Serialize, Hash)]
pub struct Album {
    pub name: String,
    pub artist_gel: uuid::Uuid,
    pub artist_indradb: uuid::Uuid,
}

const INDEX: &str = "album-unique-constraint";

#[async_trait::async_trait]
impl super::vertex::Vertex for Album {
    fn index_indradb(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind> {
        let unique_constraint = Identifier::new(INDEX)?;
        Ok(db.index_property(unique_constraint)?)
    }

    fn upsert_indradb(&self, config: &Config) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_indradb {
            return Ok(uuid::Uuid::new_v4());
        }

        let id = Identifier::new("album")?;
        let name = Identifier::new("album-name")?;
        let artist = Identifier::new("album-artist")?;
        // let albums = Identifier::new("artist-albums")?;
        let unique_constraint = Identifier::new(INDEX)?;

        let vertex = Vertex::new(id);
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash_value = hasher.finish();

        let album_artist = Edge::new(vertex.id, artist, self.artist_indradb);
        // let artist_album = Edge::new(self.artist_id, albums, vertex.id);

        let results = config.indradb.get(Query::VertexWithPropertyValue(
            VertexWithPropertyValueQuery::new(unique_constraint, ijson!(hash_value)),
        ))?;

        let vertex_id = if let QueryOutputValue::Vertices(vertices) = &results[0] {
            if vertices.len() == 1 {
                vertices[0].id
            } else {
                let vertex_id = vertex.id;
                config.indradb.bulk_insert(vec![
                    BulkInsertItem::Vertex(vertex),
                    BulkInsertItem::VertexProperty(vertex_id, name, ijson!(self.name)),
                    BulkInsertItem::Edge(album_artist),
                    // BulkInsertItem::Edge(artist_album),
                ])?;
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
            };
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
