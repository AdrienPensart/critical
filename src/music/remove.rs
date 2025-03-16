use crate::music::errors::CriticalErrorKind;

#[derive(clap::Parser)]
#[clap(about = "Remove musics")]
pub struct Remove {
    paths: Vec<String>,
}

impl Remove {
    pub async fn remove(
        &self,
        client: gel_tokio::Client,
        dry: bool,
    ) -> Result<(), CriticalErrorKind> {
        for path in &self.paths {
            if !dry {
                Box::pin(client.execute(REMOVE_PATH_QUERY, &(path,))).await?;
            }
        }
        Ok(())
    }
}

const REMOVE_PATH_QUERY: &str = "
select remove_musics_path(
    path := <str>$0
)";
