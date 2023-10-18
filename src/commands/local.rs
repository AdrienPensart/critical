use async_trait::async_trait;
use edgedb_tokio::create_client;
use clap::Subcommand;

use crate::folders::FoldersScanner;
use crate::group_dispatch::GroupDispatch;
use crate::errors::CriticalErrorKind;

#[derive(Subcommand, Debug)]
#[clap(about = "Local music management")]
pub enum Group {
    Scan(FoldersScanner),
    #[clap(about = "Clean deleted musics")]
    Clean,
    #[clap(about = "Music collection stats")]
    Stats,
    #[clap(about = "Generate a new playlist")]
    Playlist,
}

#[async_trait]
impl GroupDispatch for Group {
    async fn dispatch(self) -> Result<(), CriticalErrorKind> {
        match self {
            Group::Scan(folders_scanner) => {
                let conn = create_client().await?;
                folders_scanner.scan(&conn).await?;
                // folders_scanner.scan().await?;
                Ok(())
            },
            Group::Clean => {
                Ok(())
            },
            Group::Playlist => {
                Ok(())
            },
            Group::Stats => {
                // let stats = user_musics.stats()?;
                // println!("Musics : {}", stats.musics);
                // println!("Links : {}", stats.links);
                // println!("Artists : {}", stats.artists);
                // println!("Albums : {}", stats.albums);
                // println!("Genres : {}", stats.genres);
                // println!("Keywords : {}", stats.keywords);
                // println!("Duration : {}", stats.duration);
                Ok(())
            }
        }
    }
}
