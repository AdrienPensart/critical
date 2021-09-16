use anyhow::Result;
use clap::{AppSettings, Clap};

use crate::folders::FoldersScanner;
use crate::group_dispatch::GroupDispatch;
use crate::user::User;
use crate::user_musics::UserMusics;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Local music management")]
pub enum Group {
    Scan(FoldersScanner),
    Clean(User),
    Stats(UserMusics),
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
            Group::Stats(user_musics) => {
                let stats = user_musics.stats()?;
                println!("Stats : {:?}", stats);
                Ok(())
            }
        }
    }
}
