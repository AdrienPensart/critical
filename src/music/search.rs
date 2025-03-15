use crate::music::errors::CriticalErrorKind;
use crate::music::music_result::MusicResult;
use crate::music::MUSIC_FIELDS;
use const_format::concatcp;

use super::playlist::{OutputOptions, Playlist, PlaylistOptions};

#[derive(clap::Parser)]
#[clap(about = "Search music")]
pub struct Search {
    /// Search pattern
    pattern: String,

    /// Playlist options
    #[clap(flatten)]
    playlist_options: PlaylistOptions,

    /// Output options
    #[clap(flatten)]
    output_options: OutputOptions,
}

impl Search {
    pub async fn search(&self, client: gel_tokio::Client) -> Result<Playlist, CriticalErrorKind> {
        let musics: Vec<MusicResult> = client.query(SEARCH_QUERY, &(&self.pattern,)).await?;
        Ok(Playlist::new(&self.pattern, &musics))
    }
    pub fn output_options(&self) -> &OutputOptions {
        &self.output_options
    }
    pub fn playlist_options(&self) -> &PlaylistOptions {
        &self.playlist_options
    }
}

const SEARCH_QUERY: &str = concatcp!(
    r#"
select search(pattern := <str>$0) {
    "#,
    MUSIC_FIELDS,
    r#"
}
"#
);
