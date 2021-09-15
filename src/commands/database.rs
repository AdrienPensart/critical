use anyhow::Result;
use clap::{AppSettings, Clap};
use crate::group_dispatch::GroupDispatch;

use crate::database::Database;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Database management")]
pub enum Group {
    #[clap(about = "Create schemas")]
    CreateSchemas(Database),
    #[clap(about = "Fill schemas")]
    FillSchemas(Database),
    #[clap(about = "Drop schemas")]
    DropSchemas(Database),
    #[clap(about = "Delete and recreate schemas")]
    ClearSchemas(Database),
    #[clap(about = "Create role and database")]
    CreateRoleAndDb(Database),
    #[clap(about = "Drop database and role")]
    DropDbAndRole(Database),
}

impl GroupDispatch for Group {
    fn dispatch(self) -> Result<()> {
        match self {
            Self::CreateSchemas(database) => {
                let mut client = database.client()?;
                database.create_schemas(&mut client)
            },
            Self::FillSchemas(database) => {
                let mut client = database.client()?;
                database.fill_schemas(&mut client)
            },
            Self::DropSchemas(database) => {
                let mut admin_no_db_client = database.admin_no_db_client()?;
                database.kill_connections(&mut admin_no_db_client)?;

                let mut client = database.client()?;
                database.drop_schemas(&mut client)
            },
            Self::ClearSchemas(database) => {
                let mut admin_no_db_client = database.admin_no_db_client()?;
                database.kill_connections(&mut admin_no_db_client)?;

                let mut client = database.client()?;
                database.drop_schemas(&mut client)?;
                database.create_schemas(&mut client)?;
                database.fill_schemas(&mut client)
            },
            Self::CreateRoleAndDb(database) => {
                let mut admin_no_db_client = database.admin_no_db_client()?;
                database.create_extensions(&mut admin_no_db_client)?;
                database.create_role(&mut admin_no_db_client)?;
                database.create_database(&mut admin_no_db_client)
            },
            Self::DropDbAndRole(database) => {
                let mut admin_no_db_client = database.admin_no_db_client()?;
                database.kill_connections(&mut admin_no_db_client)?;
                database.drop_database(&mut admin_no_db_client)?;
                database.drop_role(&mut admin_no_db_client)?;
                database.drop_extensions(&mut admin_no_db_client)
            }
        }
    }
}
