use crate::errors::CriticalErrorKind;
use crate::queries::HARD_CLEAN_QUERY;

#[derive(clap::Parser, Debug)]
#[clap(about = "Get statistics")]
pub struct Clean {}

impl Clean {
    pub async fn clean(&self, dsn: String) -> Result<(), CriticalErrorKind> {
        let client = edgedb_tokio::Client::new(
            &edgedb_tokio::Builder::new()
                .dsn(&dsn)?
                // .client_security(edgedb_tokio::ClientSecurity::InsecureDevMode)
                .build_env()
                .await?,
        );

        Ok(client.execute(HARD_CLEAN_QUERY, &()).await?)
    }
}
