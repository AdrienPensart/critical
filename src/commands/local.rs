use crate::clean::Clean;
use crate::errors::CriticalErrorKind;
use crate::group_dispatch::GroupDispatch;
use crate::playlist::Playlist;
use crate::scan::Scan;
use crate::search::Search;
use crate::stats::Stats;
use async_trait::async_trait;

#[derive(clap::Subcommand, Debug)]
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
    async fn dispatch(self, dsn: String) -> Result<(), CriticalErrorKind> {
        match self {
            Group::Scan(scan) => scan.scan(dsn).await,
            Group::Clean(clean) => clean.clean(dsn).await,
            Group::Playlist(playlist) => playlist.playlist(dsn).await,
            Group::Stats(stats) => stats.print_stats(dsn).await,
            Group::Search(search) => search.search(dsn).await,
        }
    }
}
