use crate::commands::group_dispatch::GroupDispatch;
use crate::music::bests::Bests;
use crate::music::clean::Clean;
use crate::music::errors::CriticalErrorKind;
use crate::music::folders::Folders;
use crate::music::playlist::PlaylistCommand;
use crate::music::remove::Remove;
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
    Playlist(PlaylistCommand),
    #[clap(about = "Search musics")]
    Search(Search),
    #[clap(about = "Manage folders")]
    Folders(Folders),
    #[clap(about = "Remove path")]
    Remove(Remove),
    #[clap(about = "Generate bests playlists")]
    Bests(Bests),
}

#[async_trait]
impl GroupDispatch for Group {
    async fn dispatch(
        self,
        client: edgedb_tokio::Client,
        dry: bool,
    ) -> Result<(), CriticalErrorKind> {
        match self {
            Group::Scan(scan) => scan.scan(client, dry).await,
            Group::Clean(clean) => clean.clean(client, dry).await,
            Group::Playlist(playlist) => playlist.playlist(client, dry).await,
            Group::Stats(stats) => stats.print_stats(client).await,
            Group::Search(search) => search.search(client).await,
            Group::Folders(folders) => folders.folders(client).await,
            Group::Remove(remove) => remove.remove(client, dry).await,
            Group::Bests(bests) => bests.bests(client, dry).await,
        }
    }
}
