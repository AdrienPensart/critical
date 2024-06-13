use crate::errors::CriticalErrorKind;
use crate::music::music_result::MusicResult;
use crate::queries::SEARCH_QUERY;

#[derive(clap::Parser, Debug)]
#[clap(about = "Search music")]
pub struct Search {
    pattern: String,
}

impl Search {
    pub async fn search(&self, client: edgedb_tokio::Client) -> Result<(), CriticalErrorKind> {
        let musics: Vec<MusicResult> = client.query(SEARCH_QUERY, &(&self.pattern,)).await?;
        for music in musics {
            println!("{music:?}");
        }
        Ok(())
    }
}
