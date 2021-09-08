use anyhow::Result;
use clap::{AppSettings, Clap};
#[macro_use] extern crate prettytable;
use prettytable::Table;

pub mod local;
pub mod user;
pub mod commands;
pub mod music;

use crate::commands::user::Group as UserGroup;
use crate::commands::local::Group as LocalGroup;
use crate::local::scan;

#[derive(Clap, Debug)]
#[clap(name = "critical", about = "Critical Music Listening and Bot", version = "1.0", author = "Adrien P. <crunchengine@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    group: Group,
}

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
enum Group {
    #[clap(subcommand)]
    Local(LocalGroup),
    #[clap(subcommand)]
    User(UserGroup),
}


fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::parse();
    match opts.group {
        Group::Local(local) => {
            match local {
                LocalGroup::Scan(opts) => {
                    scan(opts)
                },
            }
        },
        Group::User(user) => {
            match user {
                UserGroup::Whoami(user) => {
                    let infos = user.whoami()?;
                    println!("{:?}", infos);
                }
                UserGroup::Register(user) => {
                    user.register()?;
                }
                UserGroup::Unregister(user) => {
                    user.unregister()?;
                }
                UserGroup::Login(user) => {
                    let token = user.new_token()?;
                    println!("{}", token);
                }
                UserGroup::List(admin) => {
                    let users = admin.users()?;
                    let mut table = Table::new();
                    table.add_row(row!["ID", "Email", "First Name", "Last Name", "Created", "Updated"]);
                    for user in users {
                        table.add_row(row![
                            user.id.unwrap(),
                            user.email.unwrap(),
                            user.first_name.unwrap(),
                            user.last_name.unwrap(),
                            user.created_at.unwrap(),
                            user.updated_at.unwrap(),
                        ]);
                    }
                    table.printstd();
                }
            };
            Ok(())
        }
    }
}
