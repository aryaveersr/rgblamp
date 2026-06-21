use std::io::stdout;

use anyhow::ensure;
use clap::Args;
use rgblamp::{LampArray, LampAttrs};

#[derive(Args, Debug)]
pub struct ListCommand {
    /// List lamps for a specific device only.
    #[arg(short, long = "device")]
    device_id: Option<usize>,

    /// Output as JSON.
    #[arg(short, long)]
    json: bool,
}

impl ListCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        let devices = LampArray::enumerate()?;

        if let Some(device_id) = self.device_id {
            ensure!(device_id < devices.len(), "device id out of range");

            if self.json {
                let mut handle = stdout().lock();
                serde_json::to_writer(&mut handle, &self.json_device(&devices[device_id]))?;
            } else {
                self.list_device(&devices[device_id]);
            }
        } else {
            if self.json {
                let mut handle = stdout().lock();
                let value: Vec<_> = devices.iter().map(|d| self.json_device(d)).collect();

                serde_json::to_writer(&mut handle, &value)?;
            } else {
                for (i, device) in devices.iter().enumerate() {
                    self.list_device(device);
                    if i + 1 != devices.len() {
                        println!();
                    }
                }
            }
        }

        Ok(())
    }

    fn json_device(&self, device: &LampArray) -> serde_json::Value {
        serde_json::json!({
            "id": &device.id(),
            "path": device.path(),
            "min_update_interval": &device.min_update_interval(),
            "lamps": device.lamps()
        })
    }

    fn list_device(&self, device: &LampArray) {
        println!("Device {}:", device.id());
        println!("  Path: {}", device.path().display());
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
