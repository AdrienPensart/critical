use std::hash::{DefaultHasher, Hash, Hasher};

use gel_derive::Queryable;
use indradb::{
    BulkInsertItem, Database, Identifier, MemoryDatastore, Query, QueryOutputValue, Vertex,
    VertexWithPropertyValueQuery, ijson,
};
use serde::Serialize;

use super::errors::CriticalErrorKind;
use super::{cache::UpsertCache, config::Config};

#[derive(clap::Parser)]
#[clap(about = "List folders")]
pub struct Folders {}

#[derive(Queryable)]
pub struct FolderOutput {
    name: String,
    username: String,
    ipv4: String,
    n_musics: i64,
}

#[derive(Serialize, Hash)]
pub struct Folder {
    pub name: String,
    pub username: String,
    pub ipv4: String,
}

const INDEX: &str = "folder-unique-constraint";

#[async_trait::async_trait]
impl super::vertex::Vertex for Folder {
    fn index_indradb(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind> {
        let unique_constraint = Identifier::new(INDEX)?;
        Ok(db.index_property(unique_constraint)?)
    }

    fn upsert_indradb(&self, config: &Config) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_indradb {
            return Ok(uuid::Uuid::new_v4());
        }

        let id = Identifier::new("folder")?;
        let name = Identifier::new("folder-name")?;
        let username = Identifier::new("folder-username")?;
        let ipv4 = Identifier::new("folder-ipv4")?;
        let unique_constraint = Identifier::new(INDEX)?;

        let vertex = Vertex::new(id);
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash_value = hasher.finish();

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
    async fn upsert_gel(
        &self,
        config: &Config,
        cache: &mut UpsertCache,
    ) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_gel {
            return Ok(uuid::Uuid::new_v4());
        }

        if cache.folders.contains_key(&self.name) {
            return Ok(cache.folders[&self.name]);
        }
        let mut folder_id: Option<uuid::Uuid> = None;
        for _ in 0..config.retries {
            let folder_result =
                Box::pin(config.gel.query_required_single(
                    UPSERT_FOLDER,
                    &(&self.name, &self.username, &self.ipv4),
                ))
                .await;
            match folder_result {
                Err(e) => {
                    if e.kind_name() != "TransactionSerializationError" {
                        return Err(CriticalErrorKind::GelErrorWithObject {
                            error: e,
                            object: self.name.clone(),
                        });
                    }
                    // load_music_files_bar.println(format!("retrying upsert folder {}", self.name));
                    cache.errors += 1;
                }
                Ok(id) => {
                    folder_id = Some(id);
                    break;
                }
            };
        }
        let Some(folder_id) = folder_id else {
            return Err(CriticalErrorKind::UpsertError {
                path: self.name.clone(),
                object: self.name.clone(),
            });
        };
        cache.folders.insert(self.name.clone(), folder_id);
        Ok(folder_id)
    }
}

impl FolderOutput {
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
    ) -> Result<Vec<FolderOutput>, CriticalErrorKind> {
        let folders: Vec<FolderOutput> = Box::pin(client.query(FOLDER_QUERY, &())).await?;
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

const UPSERT_FOLDER: &str = "
select upsert_folder(
    folder := <str>$0,
    username := <str>$1,
    ipv4 := <str>$2
).id";
