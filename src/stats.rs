use crate::errors::CriticalErrorKind;
use crate::queries::SELECT_FOLDERS;
use edgedb_derive::Queryable;

#[derive(clap::Parser, Debug)]
#[clap(about = "Get statistics")]
pub struct Stats {}

#[derive(Queryable)]
pub struct Folder {
    pub name: String,
    pub username: String,
    pub human_size: String,
    pub human_duration: String,
    pub ipv4: String,
    pub n_musics: i64,
    pub n_artists: i64,
    pub n_albums: i64,
    pub n_genres: i64,
    pub n_keywords: i64,
}

impl Stats {
    pub async fn print_stats(&self, dsn: String) -> Result<(), CriticalErrorKind> {
        let client = edgedb_tokio::Client::new(
            &edgedb_tokio::Builder::new()
                .dsn(&dsn)?
                // .client_security(edgedb_tokio::ClientSecurity::InsecureDevMode)
                .build_env()
                .await?,
        );

        let folders: Vec<Folder> = client.query(SELECT_FOLDERS, &()).await?;
        for folder in folders {
            println!("Folder : {}", folder.name);
            println!("Username : {}", folder.username);
            println!("IPv4 : {}", folder.ipv4);
            println!("Musics : {}", folder.n_musics);
            println!("Artists : {}", folder.n_artists);
            println!("Albums : {}", folder.n_albums);
            println!("Genres : {}", folder.n_genres);
            println!("Keywords : {}", folder.n_keywords);
            println!("Size : {}", folder.human_size);
            println!("Duration: {}", folder.human_duration);
        }
        Ok(())
    }
}
