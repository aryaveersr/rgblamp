use clap::Subcommand;

use crate::commands::{list::ListCommand, set::SetCommand};

pub mod list;
pub mod set;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Set lamp(s) to a specific color
    Set(SetCommand),

    /// List all detectable lamp(s) and their properties
    List(ListCommand),
}
