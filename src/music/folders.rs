use crate::music::errors::CriticalErrorKind;
use gel_derive::Queryable;

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

impl Folder {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn username(&self) -> &String {
        &self.username
    }
    pub fn ipv4(&self) -> &String {
        &self.ipv4
    }
    pub fn n_musics(&self) -> i64 {
        self.n_musics
    }
}

impl Folders {
    pub async fn folders(
        &self,
        client: gel_tokio::Client,
    ) -> Result<Vec<Folder>, CriticalErrorKind> {
        let folders: Vec<Folder> = client.query(FOLDER_QUERY, &()).await?;
        Ok(folders)
    }
}

const FOLDER_QUERY: &str = r#"
select Folder {
    name,
    username,
    ipv4,
    n_musics,
}
order by .name
"#;
