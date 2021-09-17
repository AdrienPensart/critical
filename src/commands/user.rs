use anyhow::Result;
use clap::{AppSettings, Clap};
use prettytable::Table;

use crate::group_dispatch::GroupDispatch;
use crate::user::User;
use crate::user::register::UserRegister;
use crate::user::login::UserLogin;
use crate::user::accounts::AdminListUsers;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "User management")]
pub enum Group {
    Register(UserRegister),
    #[clap(visible_alias = "token")]
    Login(UserLogin),
    List(AdminListUsers),

    #[clap(about = "Print information about me")]
    Whoami(User),
    #[clap(about = "Unregister user")]
    Unregister(User),
}

impl GroupDispatch for Group {
    fn dispatch(self) -> Result<()> {
        match self {
            Self::Whoami(user) => {
                let infos = user.whoami()?;
                println!("{:?}", infos);
            }
            Self::Register(user) => {
                let registered_user = user.register()?;
                registered_user.load_default_filters()?;
            }
            Self::Unregister(user) => {
                user.unregister()?;
            }
            Self::Login(user) => {
                let token = user.new_token()?;
                println!("{}", token);
            }
            Self::List(admin) => {
                let users = admin.users()?;
                let mut table = Table::new();
                table.add_row(row!["ID", "Email", "First Name", "Last Name", "Created", "Updated"]);
                for user in users {
                    table.add_row(row![
                        user.id.map_or_else(|| "N/A".to_string(), |id| id.to_string()),
                        user.email.unwrap_or_else(|| "N/A".to_string()),
                        user.first_name.unwrap_or_else(|| "N/A".to_string()),
                        user.last_name.unwrap_or_else(|| "N/A".to_string()),
                        user.created_at.unwrap_or_else(|| "N/A".to_string()),
                        user.updated_at.unwrap_or_else(|| "N/A".to_string()),
                    ]);
                }
                table.printstd();
            }
        };
        Ok(())
    }
}
