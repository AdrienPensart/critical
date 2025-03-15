use indradb::{
    ijson, BulkInsertItem, Database, Identifier, MemoryDatastore, Query, QueryOutputValue, Vertex,
    VertexWithPropertyValueQuery,
};

use super::errors::CriticalErrorKind;

pub struct KeywordVertex {
    pub name: String,
}

const KEYWORD_NAME: &str = "keyword-name";

impl KeywordVertex {
    pub fn index(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind> {
        let unique_constraint = Identifier::new(KEYWORD_NAME)?;
        Ok(db.index_property(unique_constraint)?)
    }

    pub fn upsert(&self, db: &Database<MemoryDatastore>) -> Result<uuid::Uuid, CriticalErrorKind> {
        let id = Identifier::new("keyword")?;
        let name = Identifier::new(KEYWORD_NAME)?;
        // db.index_property(name)?;

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
