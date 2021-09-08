use clap::{AppSettings, Clap};
use crate::user::User;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Local music management")]
pub enum Group {
    Scan(ScanOpts),
}

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Scan folders and save music")]
pub struct ScanOpts {
    /// Upsert chunks
    #[clap(default_value = "200", long)]
    pub chunks: usize,

    /// MusicBot user
    #[clap(flatten)]
    pub user: User,

    /// Clean musics before scanning
    #[clap(short, long)]
    pub clean: bool,
    pub folders: Vec<String>
}
