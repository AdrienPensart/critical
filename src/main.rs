#[macro_use] extern crate prettytable;
use anyhow::Result;
use clap::{AppSettings, Clap};
use enum_dispatch::enum_dispatch;

pub mod helpers;
pub mod types;
pub mod group_dispatch;
pub mod folders;
pub mod user_musics;
pub mod user_filter;
pub mod err_on_some;
pub mod user;
pub mod commands;
pub mod music;
pub mod filter;
pub mod database;

use crate::group_dispatch::GroupDispatch;
use crate::commands::user::Group as UserGroup;
use crate::commands::local::Group as LocalGroup;
use crate::commands::filter::Group as FilterGroup;
use crate::commands::database::Group as DatabaseGroup;


#[derive(Clap, Debug)]
#[clap(name = "critical", about = "Critical Music Listening and Bot", version = "1.0", author = "Adrien P. <crunchengine@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    group: Group,
}

#[enum_dispatch(GroupDispatch)]
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
enum Group {
    #[clap(subcommand)]
    Local(LocalGroup),
    #[clap(subcommand)]
    User(UserGroup),
    #[clap(subcommand)]
    Db(DatabaseGroup),
    #[clap(subcommand)]
    Filter(FilterGroup),
}

fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::parse();
    opts.group.dispatch()
}
