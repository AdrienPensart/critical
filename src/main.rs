use anyhow::Result;
use clap::{AppSettings, Clap};

pub mod local;
pub mod commands;
pub mod music;
pub mod flac_file;
pub mod mp3_file;

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
    println!("{:?}", opts);
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
                UserGroup::Register(register) => {
                    Ok(())
                }
                UserGroup::Unregister(unregister) => {
                    Ok(())
                }
                UserGroup::Login(user) => {
                    let token= user.new_token()?;
                    println!("{}", token);
                    Ok(())
                }
                UserGroup::List(list) => {
                    Ok(())
                }
            }
        }
    }
}
