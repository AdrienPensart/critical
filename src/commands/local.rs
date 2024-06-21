use crate::commands::group_dispatch::GroupDispatch;
use crate::music::clean::Clean;
use crate::music::errors::CriticalErrorKind;
use crate::music::playlist::Playlist;
use crate::music::scan::Scan;
use crate::music::search::Search;
use crate::music::stats::Stats;
use async_trait::async_trait;

#[derive(clap::Subcommand)]
#[clap(about = "Local music management")]
pub enum Group {
    Scan(Scan),
    #[clap(about = "Clean deleted musics")]
    Clean(Clean),
    #[clap(about = "Music collection stats")]
    Stats(Stats),
    #[clap(about = "Generate a new playlist")]
    Playlist(Playlist),
    #[clap(about = "Search musics")]
    Search(Search),
}

#[async_trait]
impl GroupDispatch for Group {
    async fn dispatch(self, client: edgedb_tokio::Client) -> Result<(), CriticalErrorKind> {
        match self {
            Group::Scan(scan) => scan.scan(client).await,
            Group::Clean(clean) => clean.clean(client).await,
            Group::Playlist(playlist) => playlist.playlist(client).await,
            Group::Stats(stats) => stats.print_stats(client).await,
            Group::Search(search) => search.search(client).await,
        }
    }
}
