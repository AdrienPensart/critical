use super::errors::CriticalErrorKind;
use super::{cache::UpsertCache, config::Config};
use gel_derive::Queryable;

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

pub struct Folder {
    pub name: String,
    pub username: String,
    pub ipv4: String,
}

#[async_trait::async_trait]
impl super::vertex::Vertex for Folder {
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
            }
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
