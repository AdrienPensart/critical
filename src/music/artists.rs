use indradb::{
    ijson, BulkInsertItem, Database, Identifier, MemoryDatastore, Query, QueryOutputValue, Vertex,
    VertexWithPropertyValueQuery,
};

use super::errors::CriticalErrorKind;

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

pub struct ArtistVertex {
    pub name: String,
}

const INDEX: &str = "artist-name";

impl ArtistVertex {
    pub fn index(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind> {
        let unique_constraint = Identifier::new(INDEX)?;
        Ok(db.index_property(unique_constraint)?)
    }

    pub fn upsert(&self, db: &Database<MemoryDatastore>) -> Result<uuid::Uuid, CriticalErrorKind> {
        let id = Identifier::new("artist")?;
        let name = Identifier::new(INDEX)?;

        let vertex = Vertex::new(id);

        let results = db.get(Query::VertexWithPropertyValue(
            VertexWithPropertyValueQuery::new(name, ijson!(self.name)),
        ))?;

        let vertex_id = if let QueryOutputValue::Vertices(vertices) = &results[0] {
            if vertices.len() == 1 {
                vertices[0].id
            } else {
                let vertex_id = vertex.id;
                db.bulk_insert(vec![
                    BulkInsertItem::Vertex(vertex),
                    BulkInsertItem::VertexProperty(vertex_id, name, ijson!(self.name)),
                ])?;
                vertex_id
            }
        } else {
            unreachable!();
        };
        Ok(vertex_id)
    }
}
