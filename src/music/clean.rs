use crate::music::errors::CriticalErrorKind;

#[derive(clap::Parser)]
#[clap(about = "Clean musics")]
pub struct Clean {
    soft: bool,
}

impl Clean {
    pub async fn clean(&self, client: edgedb_tokio::Client) -> Result<(), CriticalErrorKind> {
        clean(&client, self.soft).await
    }
}

pub async fn clean(client: &edgedb_tokio::Client, soft: bool) -> Result<(), CriticalErrorKind> {
    if soft {
        Ok(client.execute(SOFT_CLEAN_QUERY, &()).await?)
    } else {
        Ok(client.execute(HARD_CLEAN_QUERY, &()).await?)
    }
}

const SOFT_CLEAN_QUERY: &str = r#"
select {
    musics_deleted := count((delete Music filter not exists .folders)),
    albums_deleted := count((delete Album filter not exists .musics)),
    artists_deleted := count((delete Artist filter not exists .musics)),
    genres_deleted := count((delete Genre filter not exists .musics)),
    keywords_deleted := count((delete Keyword filter not exists .musics))
};
"#;

const HARD_CLEAN_QUERY: &str = "delete Artist;";
