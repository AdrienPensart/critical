use clap::Parser;
use enum_dispatch::enum_dispatch;

pub mod commands;
pub mod errors;
pub mod filter;
pub mod folders;
pub mod group_dispatch;
pub mod helpers;
pub mod music;
pub mod queries;

use crate::commands::local::Group as LocalGroup;
use crate::errors::CriticalErrorKind;
use crate::group_dispatch::GroupDispatch;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Opts {
    #[clap(subcommand)]
    group: Group,
}

#[enum_dispatch(GroupDispatch)]
#[derive(Parser, Debug)]
enum Group {
    #[clap(subcommand)]
    Local(LocalGroup),
}

#[tokio::main]
async fn main() -> Result<(), CriticalErrorKind> {
    env_logger::init();
    let opts = Opts::parse();
    opts.group.dispatch().await
}
