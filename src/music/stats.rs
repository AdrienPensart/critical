use crate::music::errors::CriticalErrorKind;
use gel_derive::Queryable;

#[derive(clap::Parser)]
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
    pub async fn stats(&self, client: gel_tokio::Client) -> Result<Vec<Folder>, CriticalErrorKind> {
        let folders: Vec<Folder> = Box::pin(client.query(SELECT_FOLDERS, &())).await?;
        Ok(folders)
    }
}

const SELECT_FOLDERS: &str = r"
select Folder {
    name, 
    username, 
    human_size, 
    human_duration,
    ipv4, 
    n_musics, 
    n_artists, 
    n_albums, 
    n_genres, 
    n_keywords
} order by .name";
