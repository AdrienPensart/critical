use crate::music::errors::CriticalErrorKind;
use crate::music::queries::HARD_CLEAN_QUERY;

#[derive(clap::Parser)]
#[clap(about = "Clean musics")]
pub struct Clean {}

impl Clean {
    pub async fn clean(&self, client: edgedb_tokio::Client) -> Result<(), CriticalErrorKind> {
        Ok(client.execute(HARD_CLEAN_QUERY, &()).await?)
    }
}
