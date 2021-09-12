use anyhow::Result;
use clap::{AppSettings, Clap};
use crate::group_dispatch::GroupDispatch;

use crate::database::Database;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Database management")]
pub enum Group {
    CreateSchemas(Database),
    DropSchemas(Database),
    ClearSchemas(Database),
    CreateRoleAndDb(Database),
    DropDb(Database),
}

impl GroupDispatch for Group {
    fn dispatch(self) -> Result<()> {
        match self {
            Self::CreateSchemas(database) => {
                let mut client = database.db_client()?;
                database.create_schemas(&mut client)
            },
            Self::DropSchemas(database) => {
                let mut client = database.db_client()?;
                database.drop_schemas(&mut client)
            },
            Self::ClearSchemas(database) => {
                let mut client = database.db_client()?;
                database.clear_schemas(&mut client)
            },
            Self::CreateRoleAndDb(database) => {
                let mut client = database.global_client()?;
                database.create_role_and_db(&mut client)
            },
            Self::DropDb(database) => {
                let mut client = database.global_client()?;
                database.drop_db(&mut client)
            }
        }
    }
}
