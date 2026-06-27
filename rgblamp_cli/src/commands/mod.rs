use clap::Subcommand;

use crate::commands::{
    auto::AutoCommand, effects::EffectsCommand, list::ListCommand, set::SetCommand,
};

mod auto;
mod effects;
mod list;
mod set;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Set lamp(s) to a specific color
    Set(SetCommand),

    /// List all detectable lamp(s) and their properties
    List(ListCommand),

    /// Turn auto mode on or off for device(s)
    Auto(AutoCommand),

    /// Run RGB effects
    Effects(EffectsCommand),
}
