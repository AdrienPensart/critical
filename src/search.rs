use crate::errors::CriticalErrorKind;
use crate::music::music_result::MusicResult;
use crate::queries::SEARCH_QUERY;

#[derive(clap::Parser, Debug)]
#[clap(about = "Search music")]
pub struct Search {
    pattern: String,
}

impl Search {
    pub async fn search(&self, dsn: String) -> Result<(), CriticalErrorKind> {
        let client = edgedb_tokio::Client::new(
            &edgedb_tokio::Builder::new()
                .dsn(&dsn)?
                // .client_security(edgedb_tokio::ClientSecurity::InsecureDevMode)
                .build_env()
                .await?,
        );

        let musics: Vec<MusicResult> = client.query(SEARCH_QUERY, &(&self.pattern,)).await?;
        for music in musics {
            println!("{music:?}");
        }
        Ok(())
    }
}
