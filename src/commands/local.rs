use anyhow::Result;
use clap::Parser;

use crate::folders::FoldersScanner;
use crate::group_dispatch::GroupDispatch;
use crate::user::User;
use crate::user_musics::UserMusics;

#[derive(Parser, Debug)]
#[clap(about = "Local music management")]
pub enum Group {
    Scan(FoldersScanner),
    Clean(User),
    Stats(UserMusics),
    Playlist(UserMusics),
}

impl GroupDispatch for Group {
    fn dispatch(self) -> Result<()> {
        match self {
            Group::Scan(folders_scanner) => {
                folders_scanner.scan()
            },
            Group::Clean(user) => {
                let deleted = user.clean_musics()?;
                println!("Deleted : {}", deleted);
                Ok(())
            },
            Group::Playlist(user) => {
                let playlist = user.playlist()?;
                println!("Playlist : {:?}", playlist);
                Ok(())
            },
            Group::Stats(user_musics) => {
                let stats = user_musics.stats()?;
                println!("Musics : {}", stats.musics);
                println!("Links : {}", stats.links);
                println!("Artists : {}", stats.artists);
                println!("Albums : {}", stats.albums);
                println!("Genres : {}", stats.genres);
                println!("Keywords : {}", stats.keywords);
                println!("Duration : {}", stats.duration);
                Ok(())
            }
        }
    }
}
