use crate::commands::{
    auto::AutoCommand, effect::EffectCommand, list::ListCommand, set::SetCommand,
};

mod auto;
mod effect;
mod list;
mod set;

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Set lamp(s) to a specific color
    Set(SetCommand),

    /// List all detectable lamp(s) and their properties
    List(ListCommand),

    /// Turn auto mode on or off for device(s)
    Auto(AutoCommand),

    /// Run RGB effects
    Effect(EffectCommand),
}
