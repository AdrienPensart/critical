use clap::{AppSettings, Clap};

use crate::user::whoami::UserWhoami;
use crate::user::register::UserRegister;
use crate::user::unregister::UserUnregister;
use crate::user::login::UserLogin;
use crate::user::accounts::AdminListUsers;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "User management")]
pub enum Group {
    Whoami(UserWhoami),
    Register(UserRegister),
    Unregister(UserUnregister),
    Login(UserLogin),
    List(AdminListUsers),
}
