use crate::commands::group_dispatch::GroupDispatch;
use crate::music::bests::Bests;
use crate::music::clean::Clean;
use crate::music::errors::CriticalErrorKind;
use crate::music::folders::Folders;
use crate::music::playlist::{OutputOptions, PlaylistCommand};
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
    async fn dispatch(self, client: gel_tokio::Client, dry: bool) -> Result<(), CriticalErrorKind> {
        match self {
            Group::Scan(scan_cmd) => scan_cmd.scan(client, dry).await,
            Group::Clean(clean_cmd) => clean_cmd.clean(client, dry).await,
            Group::Playlist(playlist_cmd) => {
                let playlist = playlist_cmd.playlist(client).await?;
                playlist.generate(
                    playlist_cmd.output_options(),
                    playlist_cmd.playlist_options(),
                    dry,
                )
            }
            Group::Stats(stats_cmd) => {
                let folders = stats_cmd.stats(client).await?;
                for folder in folders {
                    println!("Folder : {}", folder.name);
                    println!("Username : {}", folder.username);
                    println!("IPv4 : {}", folder.ipv4);
                    println!("Musics : {}", folder.n_musics);
                    println!("Artists : {}", folder.n_artists);
                    println!("Albums : {}", folder.n_albums);
                    println!("Genres : {}", folder.n_genres);
                    println!("Keywords : {}", folder.n_keywords);
                    println!("Size : {}", folder.human_size);
                    println!("Duration: {}", folder.human_duration);
                }
                Ok(())
            }
            Group::Search(search_cmd) => search_cmd.search(client).await,
            Group::Folders(folders_cmd) => {
                let folders = folders_cmd.folders(client).await?;
                for folder in folders {
                    println!("Folder : {}", folder.name());
                    println!("Username : {}", folder.username());
                    println!("IPv4 : {}", folder.ipv4());
                    println!("Musics : {}", folder.n_musics());
                }
                Ok(())
            }
            Group::Remove(remove_cmd) => remove_cmd.remove(client, dry).await,
            Group::Bests(bests_cmd) => {
                let playlists = bests_cmd.bests(client).await?;
                for playlist in playlists.iter() {
                    if (playlist.len() as u64) < bests_cmd.min_playlist_size() {
                        eprintln!(
                            "{} : size < {}",
                            playlist.name(),
                            bests_cmd.min_playlist_size()
                        );
                        continue;
                    }
                    let output_options = if let Some(out) = bests_cmd.output_options().out() {
                        OutputOptions::new(
                            bests_cmd.output_options().output(),
                            &Some(format!("{}/{}.m3u", out, playlist.name())),
                        )
                    } else {
                        bests_cmd.output_options().clone()
                    };
                    eprintln!("\nGenerating {} : {}", playlist.name(), playlist.len());
                    playlist.generate(&output_options, bests_cmd.playlist_options(), dry)?;
                }
                Ok(())
            }
        }
    }
}
