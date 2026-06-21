use anyhow::{Context, ensure};
use clap::{Args, Parser, Subcommand};
use color::{Srgb, parse_color};
use rgblamp::LampArray;

/// Control HID LampArray devices on Linux.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Set all lamps, or a particular device/lamp to a specific color.
    Set(SetArgs),
}

#[derive(Args, Debug)]
struct SetArgs {
    /// Color as any value supported by CSS. (Hex, RGB, named values, etc.)
    color: String,

    /// Device ID. Required for Lamp ID if there are multiple devices.
    #[arg(short, long = "device")]
    device_id: Option<usize>,

    /// Lamp ID.
    #[arg(short, long = "lamp")]
    lamp_id: Option<u32>,
}

fn main() -> anyhow::Result<()> {
    let args = AppArgs::parse();
    let mut devices = LampArray::enumerate()?;

    match args.command {
        Command::Set(SetArgs {
            color,
            device_id,
            lamp_id,
        }) => {
            ensure!(!devices.is_empty(), "no devices found.");

            let color = parse_color(&color)
                .map(|c| c.to_alpha_color::<Srgb>().to_rgba8())
                .context("invalid color")?;

            match (device_id, lamp_id) {
                (None, None) => {
                    for device in &mut devices {
                        device.set_all_lamps(color)?;
                    }
                }
                (None, Some(lamp_id)) => {
                    ensure!(devices.len() == 1, "multiple devices found, need device id");
                    devices[0].set_lamp(lamp_id, color)?;
                }
                (Some(device_id), None) => {
                    ensure!(device_id < devices.len(), "device id out of range");
                    devices[device_id].set_all_lamps(color)?;
                }
                (Some(device_id), Some(lamp_id)) => {
                    ensure!(device_id < devices.len(), "device id out of range");
                    devices[device_id].set_lamp(lamp_id, color)?;
                }
            }
        }
    }

    Ok(())
}
