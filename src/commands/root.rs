use crate::commands::group_dispatch::GroupDispatch;
use crate::commands::local::Group;
use crate::music::config::Config;
use crate::music::errors::CriticalErrorKind;

use clap::Parser;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(GroupDispatch)]
#[derive(Parser)]
pub enum Root {
    #[clap(subcommand)]
    Local(Group),
}
