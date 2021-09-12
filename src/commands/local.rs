use anyhow::Result;
use clap::{AppSettings, Clap};
use crate::folders::FoldersScanner;
use crate::group_dispatch::GroupDispatch;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Local music management")]
pub enum Group {
    Scan(FoldersScanner),
}

impl GroupDispatch for Group {
    fn dispatch(self) -> Result<()> {
        match self {
            Group::Scan(folders_scanner) => {
                folders_scanner.scan()
            },
        }
    }
}
