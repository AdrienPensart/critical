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
        for path in self.paths.iter() {
            if !dry {
                client.execute(REMOVE_PATH_QUERY, &(path,)).await?;
            }
        }
        Ok(())
    }
}

const REMOVE_PATH_QUERY: &str = r#"
update Music
filter contains(.paths, <str>$0)
set {folders := (select .folders filter @path != <str>$0)};
"#;
