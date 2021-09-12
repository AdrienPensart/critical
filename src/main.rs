#[macro_use] extern crate prettytable;
use anyhow::Result;
use clap::{AppSettings, Clap};
use enum_dispatch::enum_dispatch;

pub mod group_dispatch;
pub mod folders;
pub mod err_on_some;
pub mod user;
pub mod commands;
pub mod music;

use crate::group_dispatch::GroupDispatch;
use crate::commands::user::Group as UserGroup;
use crate::commands::local::Group as LocalGroup;


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
}


fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::parse();
    opts.group.dispatch()
}
