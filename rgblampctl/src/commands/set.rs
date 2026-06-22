use anyhow::{Context, ensure};
use clap::Args;
use color::{Srgb, parse_color};
use rgblamp::LampArray;

#[derive(Args, Debug)]
pub struct SetCommand {
    /// Color as any value supported by CSS (hex, rgb(), named values, etc.)
    color: String,

    /// Limit the change to a specific device
    #[arg(short, long = "device")]
    device_id: Option<usize>,

    /// Set the color of a specific lamp. Requires device id if there are multiple devices
    #[arg(short, long = "lamp")]
    lamp_id: Option<u32>,
}

impl SetCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        let mut devices = LampArray::enumerate()?;

        ensure!(!devices.is_empty(), "no devices found.");

        let color = parse_color(&self.color)
            .map(|c| c.to_alpha_color::<Srgb>().to_rgba8())
            .context("invalid color")?;

        match (self.device_id, self.lamp_id) {
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

        Ok(())
    }
}
