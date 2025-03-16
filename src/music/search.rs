use super::errors::CriticalErrorKind;
use super::music::MUSIC_FIELDS;
use super::music_result::MusicResult;
use super::playlist::{OutputOptions, Playlist, PlaylistOptions};
use const_format::concatcp;

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
        let musics: Vec<MusicResult> =
            Box::pin(client.query(SEARCH_QUERY, &(&self.pattern,))).await?;
        Ok(Playlist::new(&self.pattern, &musics))
    }
    #[must_use]
    pub fn output_options(&self) -> &OutputOptions {
        &self.output_options
    }
    #[must_use]
    pub fn playlist_options(&self) -> &PlaylistOptions {
        &self.playlist_options
    }
}

const SEARCH_QUERY: &str = concatcp!(
    "
select search(pattern := <str>$0) {
    ",
    MUSIC_FIELDS,
    "
}
"
);
