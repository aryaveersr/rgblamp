use std::io::stdout;

use rgblamp::{LampArray, LampAttrs};

use crate::device::{DeviceArgs, HidInfo};

#[derive(clap::Args, Debug)]
pub struct ListCommand {
    #[command(flatten)]
    device: DeviceArgs,

    /// Output as JSON.
    #[arg(short, long)]
    json: bool,
}

impl ListCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        let devices = self.device.iter()?;

        if self.json {
            let value = devices
                .map(|d| d.map(|d| self.json_device(d)))
                .collect::<anyhow::Result<Vec<_>>>()?;

            let mut handle = stdout().lock();
            serde_json::to_writer(&mut handle, &value)?;
        } else {
            for (i, device) in devices.enumerate() {
                self.list_device(i, device?);
            }
        }

        Ok(())
    }

    fn json_device(&self, (info, device): (HidInfo, LampArray)) -> serde_json::Value {
        serde_json::json!({
            "dev_name": device.dev_name(),
            "hid_info": info,
            "min_update_interval": &device.min_update_interval(),
            "lamps": device.lamps()
        })
    }

    fn list_device(&self, i: usize, (info, device): (HidInfo, LampArray)) {
        println!("Device {i}:");
        println!("  Dev name: {}", device.dev_name());
        println!("  Vendor: {}", info.id.vendor);
        println!("  Product: {}", info.id.product);
        println!("  Number of lamps: {}", device.lamps().len());
        println!(
            "  Minimum interval between updates: {:?}",
            device.min_update_interval()
        );
        println!("  Lamps:");

        for lamp in device.lamps() {
            self.list_lamp(lamp);
        }
    }

    fn list_lamp(&self, lamp: &LampAttrs) {
        println!("  - Lamp {}", lamp.lamp_id);
        println!(
            "    Supported color levels (rgb): {} {} {}",
            lamp.red_level_count, lamp.green_level_count, lamp.blue_level_count
        );
        println!(
            "    Supported intensity levels: {}",
            lamp.intensity_level_count
        );
        println!("    Is Programmable: {}", lamp.programmable);
    }
}
