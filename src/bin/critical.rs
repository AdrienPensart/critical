use clap::Parser;
use critical::commands::opts::Opts;
use critical::music::errors::CriticalErrorKind;

#[tokio::main]
// #[tokio::main(flavor = "current_thread")]
// #[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<(), CriticalErrorKind> {
    env_logger::init();
    let opts = Opts::parse();
    if let Err(e) = opts.dispatch().await {
        eprintln!("{e:?}");
    }
    Ok(())
}
