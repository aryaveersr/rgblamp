use clap::Subcommand;

use crate::commands::set::SetCommand;

pub mod set;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Set all lamps, or a particular device/lamp to a specific color.
    Set(SetCommand),
}
