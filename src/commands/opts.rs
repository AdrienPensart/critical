use crate::commands::group_dispatch::GroupDispatch;
use crate::commands::root::Root;
use crate::commands::{DEFAULT_DATASTORE_FILE, DEFAULT_DSN};
use crate::music::errors::CriticalErrorKind;
use clap::Parser;
use homedir::my_home;
use std::path::PathBuf;
use std::process::exit;

fn datastore_file_path(datastore: &str) -> String {
    let Ok(Some(mut home)) = my_home() else {
        exit(1);
    };
    home.push(datastore);
    home.to_string_lossy().to_string()
}

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Opts {
    #[clap(subcommand)]
    root: Root,

    #[clap(
        long,
        global = true,
        default_value_t = DEFAULT_DSN.to_string()
    )]
    /// EdgeDB DSN
    pub dsn: String,

    #[clap(long, global = true)]
    /// Dry mode
    pub dry: bool,

    #[clap(
        long,
        global = true,
        default_value_t = datastore_file_path(DEFAULT_DATASTORE_FILE)
    )]
    /// Datastore path
    pub datastore: String,

    #[clap(long, global = true)]
    /// Disable Gel DB
    pub no_gel: bool,

    #[clap(long, global = true)]
    /// Disable Indra DB
    pub no_indradb: bool,
}

impl Opts {
    pub async fn dispatch(self) -> Result<(), CriticalErrorKind> {
        let config = gel_tokio::Builder::new()
            .dsn(&self.dsn)
            // .client_security(edgedb_tokio::ClientSecurity::InsecureDevMode)
            .build()?;
        let client = gel_tokio::Client::new(&config);
        let config = Config {
            client,
            dry: self.dry,
            datastore: self.datastore.into(),
            no_gel: self.no_gel,
            no_indradb: self.no_indradb,
        };
        self.root.dispatch(config).await
    }
}

pub struct Config {
    pub client: gel_tokio::Client,
    pub dry: bool,
    pub datastore: PathBuf,
    pub no_gel: bool,
    pub no_indradb: bool,
}
