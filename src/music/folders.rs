use std::hash::{DefaultHasher, Hash, Hasher};

use crate::music::errors::CriticalErrorKind;
use gel_derive::Queryable;
use indradb::{
    ijson, BulkInsertItem, Database, Identifier, MemoryDatastore, Query, QueryOutputValue, Vertex,
    VertexWithPropertyValueQuery,
};
use serde::Serialize;

#[derive(clap::Parser)]
#[clap(about = "List folders")]
pub struct Folders {}

#[derive(Queryable)]
pub struct Folder {
    name: String,
    username: String,
    ipv4: String,
    n_musics: i64,
}

#[derive(Serialize, Hash)]
pub struct FolderVertex {
    pub name: String,
    pub username: String,
    pub ipv4: String,
}

const INDEX: &str = "folder-unique-constraint";

impl FolderVertex {
    pub fn index(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind> {
        let unique_constraint = Identifier::new(INDEX)?;
        Ok(db.index_property(unique_constraint)?)
    }

    pub fn upsert(&self, db: &Database<MemoryDatastore>) -> Result<uuid::Uuid, CriticalErrorKind> {
        let id = Identifier::new("folder")?;
        let name = Identifier::new("folder-name")?;
        let username = Identifier::new("folder-username")?;
        let ipv4 = Identifier::new("folder-ipv4")?;
        let unique_constraint = Identifier::new(INDEX)?;

        let vertex = Vertex::new(id);
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash_value = hasher.finish();

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
                    BulkInsertItem::VertexProperty(vertex_id, username, ijson!(self.username)),
                    BulkInsertItem::VertexProperty(vertex_id, ipv4, ijson!(self.ipv4)),
                    BulkInsertItem::VertexProperty(
                        vertex_id,
                        unique_constraint,
                        ijson!(hash_value),
                    ),
                ])?;
                vertex_id
            }
        } else {
            unreachable!();
        };
        Ok(vertex_id)
    }
}

impl Folder {
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }
    #[must_use]
    pub fn username(&self) -> &String {
        &self.username
    }
    #[must_use]
    pub fn ipv4(&self) -> &String {
        &self.ipv4
    }
    #[must_use]
    pub fn n_musics(&self) -> i64 {
        self.n_musics
    }
}

impl Folders {
    pub async fn folders(
        &self,
        client: gel_tokio::Client,
    ) -> Result<Vec<Folder>, CriticalErrorKind> {
        let folders: Vec<Folder> = Box::pin(client.query(FOLDER_QUERY, &())).await?;
        Ok(folders)
    }
}

const FOLDER_QUERY: &str = r"
select Folder {
    name,
    username,
    ipv4,
    n_musics,
}
order by .name
";
