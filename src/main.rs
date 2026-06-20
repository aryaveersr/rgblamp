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

fn main() {
    let args = AppArgs::parse();
    let mut devices = LampArray::enumerate();

    match args.command {
        Command::Set(SetArgs {
            color,
            device_id,
            lamp_id,
        }) => {
            if devices.is_empty() {
                println!("No devices found.");
                return;
            }

            let color = match parse_color(&color) {
                Ok(color) => color.to_alpha_color::<Srgb>(),
                Err(err) => {
                    println!("Invalid color: {err}.");
                    return;
                }
            };

            match (device_id, lamp_id) {
                (None, None) => {
                    for device in &mut devices {
                        device.set_color_all(color);
                    }
                }
                (None, Some(lamp_id)) => {
                    if devices.len() != 1 {
                        println!(
                            "Multiple devices found. You need to specify Device ID along with Lamp ID"
                        );
                        return;
                    } else if devices[0].attrs().lamp_count <= lamp_id {
                        println!("Lamp ID out of range.");
                    } else {
                        devices[0].set_color_lamp(lamp_id, color);
                    }
                }
                (Some(device_id), None) => {
                    if device_id >= devices.len() {
                        println!("Device ID out of range.");
                        return;
                    } else {
                        devices[device_id].set_color_all(color);
                    }
                }
                (Some(device_id), Some(lamp_id)) => {
                    if device_id >= devices.len() {
                        println!("Device ID out of range.");
                        return;
                    } else if devices[device_id].attrs().lamp_count <= lamp_id {
                        println!("Lamp ID out of range.");
                    } else {
                        devices[device_id].set_color_lamp(lamp_id, color);
                    }
                }
            }
        }
    }
}
