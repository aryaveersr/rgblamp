use clap::Parser;
use clap_verbosity_flag::Verbosity;
use log::trace;

use crate::commands::Commands;

mod commands;

/// Control HID LampArray devices on Linux.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    verbosity: Verbosity,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    trace!("Running with args {args:?}");

    match args.command {
        Commands::Set(set_command) => set_command.exec(),
        Commands::List(list_command) => list_command.exec(),
        Commands::Auto(auto_command) => auto_command.exec(),
    }
}
