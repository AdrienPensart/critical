use clap::Parser;
use enum_dispatch::enum_dispatch;

pub mod clean;
pub mod commands;
pub mod errors;
pub mod filter;
pub mod group_dispatch;
pub mod helpers;
pub mod music;
pub mod playlist;
pub mod queries;
pub mod scan;
pub mod search;
pub mod stats;

use crate::commands::local::Group as LocalGroup;
use crate::errors::CriticalErrorKind;
use crate::group_dispatch::GroupDispatch;

const DEFAULT_DSN: &str = "edgedb://musicbot:musicbot@127.0.0.1:5656/main?tls_security=insecure";

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Opts {
    #[clap(subcommand)]
    group: Group,

    #[clap(
        long,
        global = true,
        default_value_t = DEFAULT_DSN.to_string()
    )]
    /// EdgeDB DSN
    pub dsn: String,
}

#[enum_dispatch(GroupDispatch)]
#[derive(Parser, Debug)]
enum Group {
    #[clap(subcommand)]
    Local(LocalGroup),
}

#[tokio::main]
// #[tokio::main(flavor = "current_thread")]
// #[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<(), CriticalErrorKind> {
    env_logger::init();
    let opts = Opts::parse();
    if let Err(e) = opts.group.dispatch(opts.dsn).await {
        eprintln!("{e}");
    }
    Ok(())
}
