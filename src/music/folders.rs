use crate::music::errors::CriticalErrorKind;
use edgedb_derive::Queryable;

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

impl Folders {
    pub async fn folders(&self, client: edgedb_tokio::Client) -> Result<(), CriticalErrorKind> {
        let folders: Vec<Folder> = client.query(FOLDER_QUERY, &()).await?;
        for folder in folders {
            println!("Folder : {}", folder.name);
            println!("Username : {}", folder.username);
            println!("IPv4 : {}", folder.ipv4);
            println!("Musics : {}", folder.n_musics);
        }
        Ok(())
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
