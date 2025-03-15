use std::hash::{DefaultHasher, Hash, Hasher};

use indradb::{
    ijson, BulkInsertItem, Database, Edge, Identifier, MemoryDatastore, Query, QueryOutputValue,
    Vertex, VertexWithPropertyValueQuery,
};
use serde::Serialize;

use super::errors::CriticalErrorKind;

#[derive(Serialize, Hash)]
pub struct AlbumVertex {
    pub name: String,
    pub artist_id: uuid::Uuid,
}

const INDEX: &str = "album-unique-constraint";

impl AlbumVertex {
    pub fn index(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind> {
        let unique_constraint = Identifier::new(INDEX)?;
        Ok(db.index_property(unique_constraint)?)
    }
    pub fn upsert(&self, db: &Database<MemoryDatastore>) -> Result<uuid::Uuid, CriticalErrorKind> {
        let id = Identifier::new("album")?;
        let name = Identifier::new("album-name")?;
        let artist = Identifier::new("album-artist")?;
        // let albums = Identifier::new("artist-albums")?;
        let unique_constraint = Identifier::new(INDEX)?;

        let vertex = Vertex::new(id);
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash_value = hasher.finish();

        let album_artist = Edge::new(vertex.id, artist, self.artist_id);
        // let artist_album = Edge::new(self.artist_id, albums, vertex.id);

        let results = db.get(Query::VertexWithPropertyValue(
            VertexWithPropertyValueQuery::new(unique_constraint, ijson!(hash_value)),
        ))?;

        let vertex_id = if let QueryOutputValue::Vertices(vertices) = &results[0] {
            if vertices.len() == 1 {
                vertices[0].id
            } else {
                let vertex_id = vertex.id;
                db.bulk_insert(vec![
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
}
