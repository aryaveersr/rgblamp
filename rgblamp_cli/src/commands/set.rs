use anyhow::{Context, ensure};
use color::{Srgb, parse_color};

use crate::device::DeviceArgs;

#[derive(clap::Args, Debug)]
pub struct SetCommand {
    /// Color as any value supported by CSS (hex, rgb(), named values, etc.)
    color: String,

    /// Set the color of a specific lamp.
    #[arg(short, long = "lamp")]
    lamp_id: Option<u32>,

    #[command(flatten)]
    device: DeviceArgs,
}

impl SetCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        let mut devices = self.device.iter()?.peekable();
        let color = parse_color(&self.color)
            .map(|c| c.to_alpha_color::<Srgb>().to_rgba8())
            .context("invalid color")?;

        ensure!(devices.peek().is_some(), "no devices found");

        for device in devices {
            let (_, mut device) = device?;

            match self.lamp_id {
                Some(lamp_id) => {
                    device.set_lamp(lamp_id, color)?;
                }
                None => {
                    device.set_all_lamps(color)?;
                }
            }
        }

        Ok(())
    }
}
