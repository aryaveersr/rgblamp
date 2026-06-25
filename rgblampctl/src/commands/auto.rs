use anyhow::ensure;
use clap::{Args, ValueEnum};
use rgblamp::lamparrays::LampArrays;

#[derive(Args, Debug)]
pub struct AutoCommand {
    /// Turn auto mode on/off
    #[arg(value_enum)]
    value: Switch,

    /// Limit the change to a specific device
    #[arg(short, long = "device")]
    device_id: Option<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Switch {
    On,
    Off,
}

impl AutoCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        let mut devices = LampArrays::new()?;
        let auto_mode = self.value == Switch::On;

        ensure!(!devices.is_empty(), "no devices found.");

        match self.device_id {
            Some(device_id) => {
                ensure!(device_id < devices.len(), "device id out of range");
                devices[device_id].set_auto_mode(auto_mode)?;
            }
            None => {
                for device in devices.iter_mut() {
                    device.set_auto_mode(auto_mode)?;
                }
            }
        }

        Ok(())
    }
}
