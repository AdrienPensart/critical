use super::errors::CriticalErrorKind;

#[derive(clap::Parser)]
#[clap(about = "Clean musics")]
pub struct Clean {
    /// Delete only orphan objects
    #[clap(short, long)]
    soft: bool,
}

impl Clean {
    pub async fn clean(
        &self,
        client: gel_tokio::Client,
        dry: bool,
    ) -> Result<(), CriticalErrorKind> {
        clean(&client, self.soft, dry).await
    }
}

pub async fn clean(
    client: &gel_tokio::Client,
    soft: bool,
    dry: bool,
) -> Result<(), CriticalErrorKind> {
    let query = if soft {
        SOFT_CLEAN_QUERY
    } else {
        HARD_CLEAN_QUERY
    };

    if dry {
        Ok(())
    } else {
        Ok(Box::pin(client.execute(query, &())).await?)
    }
}

const SOFT_CLEAN_QUERY: &str = r"
select {
    musics_deleted := count((delete Music filter not exists .folders)),
    albums_deleted := count((delete Album filter not exists .musics)),
    artists_deleted := count((delete Artist filter not exists .musics)),
    genres_deleted := count((delete Genre filter not exists .musics)),
    keywords_deleted := count((delete Keyword filter not exists .musics))
};
";

const HARD_CLEAN_QUERY: &str = "delete Artist;";
