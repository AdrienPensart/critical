use crate::errors::CriticalErrorKind;
use crate::queries::HARD_CLEAN_QUERY;

#[derive(clap::Parser, Debug)]
#[clap(about = "Get statistics")]
pub struct Clean {}

impl Clean {
    pub async fn clean(&self, client: edgedb_tokio::Client) -> Result<(), CriticalErrorKind> {
        Ok(client.execute(HARD_CLEAN_QUERY, &()).await?)
    }
}
