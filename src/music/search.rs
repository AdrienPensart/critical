use crate::music::errors::CriticalErrorKind;
use crate::music::music_result::MusicResult;
use crate::music::MUSIC_FIELDS;
use const_format::concatcp;

#[derive(clap::Parser)]
#[clap(about = "Search music")]
pub struct Search {
    pattern: String,
}

impl Search {
    pub async fn search(&self, client: gel_tokio::Client) -> Result<(), CriticalErrorKind> {
        let musics: Vec<MusicResult> = client.query(SEARCH_QUERY, &(&self.pattern,)).await?;
        for music in musics {
            println!("{music:?}");
        }
        Ok(())
    }
}

const SEARCH_QUERY: &str = concatcp!(
    r#"
select Music {
    "#,
    MUSIC_FIELDS,
    r#"
}
filter
    .name ilike <str>$0 or
    .genre.name ilike <str>$0 or
    .album.name ilike <str>$0 or
    .artist.name ilike <str>$0 or
    .keywords.name ilike <str>$0
order by 
    .artist.name then
    .album.name then
    .track then
    .name
"#
);
