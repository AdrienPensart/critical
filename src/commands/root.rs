use crate::commands::group_dispatch::GroupDispatch;
use crate::commands::local::Group;
use crate::commands::DEFAULT_DSN;
use crate::music::errors::CriticalErrorKind;

use clap::Parser;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(GroupDispatch)]
#[derive(Parser)]
pub enum Root {
    #[clap(subcommand)]
    Local(Group),
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
}

impl Opts {
    pub async fn dispatch(self) -> Result<(), CriticalErrorKind> {
        let client = edgedb_tokio::Client::new(
            &edgedb_tokio::Builder::new()
                .dsn(&self.dsn)?
                // .client_security(edgedb_tokio::ClientSecurity::InsecureDevMode)
                .build_env()
                .await?,
        );

        self.root.dispatch(client).await
    }
}
