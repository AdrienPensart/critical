use anyhow::Result;
use clap::{AppSettings, Clap};
use crate::group_dispatch::GroupDispatch;

use crate::database::Database;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Database management")]
pub enum Group {
    #[clap(about = "Create musicbot")]
    Create(Database),
    #[clap(about = "Delete musicbot")]
    Drop(Database),
    #[clap(about = "Delete and recreate schemas")]
    Clear(Database),
}

impl GroupDispatch for Group {
    fn dispatch(self) -> Result<()> {
        match self {
            Self::Create(database) => {
                let mut admin_no_db_client = database.admin_no_db_client()?;
                database.create_role(&mut admin_no_db_client)?;
                database.create_database(&mut admin_no_db_client)?;

                let mut admin_client = database.admin_client()?;
                database.create_extensions(&mut admin_client)?;

                let mut client = database.client()?;
                database.create_schemas(&mut client)?;
                database.fill_schemas(&mut client)?;
                Ok(())
            },
            Self::Drop(database) => {
                let mut admin_no_db_client = database.admin_no_db_client()?;
                database.kill_connections(&mut admin_no_db_client)?;
                database.drop_database(&mut admin_no_db_client)?;
                database.drop_role(&mut admin_no_db_client)?;
                database.drop_extensions(&mut admin_no_db_client)?;
                Ok(())
            },
            Self::Clear(database) => {
                let mut admin_no_db_client = database.admin_no_db_client()?;
                database.kill_connections(&mut admin_no_db_client)?;

                let mut client = database.client()?;
                database.drop_schemas(&mut client)?;
                database.create_schemas(&mut client)?;
                database.fill_schemas(&mut client)?;
                Ok(())
            },
        }
    }
}
