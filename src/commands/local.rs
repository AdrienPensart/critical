use clap::{AppSettings, Clap};
use crate::folders::FoldersScanner;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Local music management")]
pub enum Group {
    Scan(FoldersScanner),
}
