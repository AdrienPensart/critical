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
