use clap::Parser;

use crate::commands::Commands;

mod commands;

/// Control HID LampArray devices on Linux.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> anyhow::Result<()> {
    let args = AppArgs::parse();

    match args.command {
        Commands::Set(set_command) => set_command.exec(),
    }
}
