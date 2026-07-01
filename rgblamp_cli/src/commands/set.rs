use anyhow::{Context, ensure};
use color::{Srgb, parse_color};

use crate::{
    device::DeviceArgs,
    lamp_id::{LampIdArg, LampIdItem},
};

#[derive(clap::Args, Debug)]
pub struct SetCommand {
    /// Color as any value supported by CSS (hex, rgb(), named values, etc.)
    color: String,

    /// Set the color of specific lamp(s). Can be comma separated ids or id ranges.
    /// (eg: 0, 4..=5, 11..13, etc.)
    #[arg(short, long = "lamp")]
    lamp_ids: LampIdArg,

    #[command(flatten)]
    device: DeviceArgs,
}

impl SetCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        let mut devices = self.device.enumerate()?;
        let color = parse_color(&self.color)
            .map(|c| c.to_alpha_color::<Srgb>().to_rgba8())
            .context("invalid color")?;

        ensure!(!devices.is_empty(), "no devices found");

        for (_, device) in &mut devices {
            if self.lamp_ids.is_empty() {
                device.set_all_lamps(color)?;
            } else {
                let mut builder = device.builder();

                for item in &self.lamp_ids {
                    match item? {
                        LampIdItem::Id(id) => builder.set(id, color)?,
                        LampIdItem::Range(range) => builder.set_range(range, color)?,
                    };
                }

                builder.finish()?;
            }
        }

        Ok(())
    }
}
