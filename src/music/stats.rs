use super::{config::Config, errors::CriticalErrorKind};
use gel_derive::Queryable;

#[derive(clap::Parser)]
#[clap(about = "Get statistics")]
pub struct Stats {}

#[derive(Queryable)]
pub struct FolderOutput {
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
    pub async fn stats(&self, config: Config) -> Result<Vec<FolderOutput>, CriticalErrorKind> {
        if !config.no_gel {
            let folders: Vec<FolderOutput> =
                Box::pin(config.gel.query(SELECT_FOLDERS, &())).await?;
            return Ok(folders);
        }
        Ok(vec![])
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
