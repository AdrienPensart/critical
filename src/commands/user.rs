use clap::{AppSettings, Clap};

use crate::user::User;
use crate::user::register::UserRegister;
use crate::user::login::UserLogin;
use crate::user::accounts::AdminListUsers;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "User management")]
pub enum Group {
    Register(UserRegister),
    Login(UserLogin),
    List(AdminListUsers),

    #[clap(about = "Print information about me")]
    Whoami(User),
    #[clap(about = "Unregister user")]
    Unregister(User),
}
