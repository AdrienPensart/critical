use indradb::{
    BulkInsertItem, Database, Identifier, MemoryDatastore, Query, QueryOutputValue, Vertex,
    VertexWithPropertyValueQuery, ijson,
};

use super::{cache::UpsertCache, config::Config, errors::CriticalErrorKind};

pub struct Keyword {
    pub name: String,
}

const KEYWORD_NAME: &str = "keyword-name";

#[async_trait::async_trait]
impl super::vertex::Vertex for Keyword {
    fn index_indradb(db: &Database<MemoryDatastore>) -> Result<(), CriticalErrorKind> {
        let unique_constraint = Identifier::new(KEYWORD_NAME)?;
        Ok(db.index_property(unique_constraint)?)
    }

    fn upsert_indradb(&self, config: &Config) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_indradb {
            return Ok(uuid::Uuid::new_v4());
        }

        let id = Identifier::new("keyword")?;
        let name = Identifier::new(KEYWORD_NAME)?;
        // db.index_property(name)?;

        let vertex = Vertex::new(id);

        let results = config.indradb.get(Query::VertexWithPropertyValue(
            VertexWithPropertyValueQuery::new(name, ijson!(self.name)),
        ))?;

        let vertex_id = if let QueryOutputValue::Vertices(vertices) = &results[0] {
            if vertices.len() == 1 {
                vertices[0].id
            } else {
                let vertex_id = vertex.id;
                config.indradb.bulk_insert(vec![
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

    async fn upsert_gel(
        &self,
        config: &Config,
        cache: &mut UpsertCache,
    ) -> Result<uuid::Uuid, CriticalErrorKind> {
        if config.no_gel {
            return Ok(uuid::Uuid::new_v4());
        }

        if cache.keywords.contains_key(&self.name) {
            return Ok(cache.keywords[&self.name]);
        }
        let mut keyword_id: Option<uuid::Uuid> = None;
        for _ in 0..config.retries {
            let keyword_folder = Box::pin(
                config
                    .gel
                    .query_required_single(UPSERT_KEYWORD, &(&self.name,)),
            )
            .await;
            match keyword_folder {
                Err(e) => {
                    if e.kind_name() != "TransactionSerializationError" {
                        return Err(CriticalErrorKind::GelErrorWithObject {
                            error: e,
                            object: self.name.clone(),
                        });
                    }
                    // load_music_files_bar.println(format!("retrying upsert keyword {}", self.name));
                    cache.errors += 1;
                }
                Ok(id) => {
                    keyword_id = Some(id);
                    break;
                }
            };
        }
        let Some(keyword_id) = keyword_id else {
            return Err(CriticalErrorKind::UpsertError {
                path: self.name.clone(),
                object: self.name.clone(),
            });
        };
        cache.keywords.insert(self.name.clone(), keyword_id);
        Ok(keyword_id)
    }
}

const UPSERT_KEYWORD: &str = "
select upsert_keyword(
    keyword := <str>$0
).id";
