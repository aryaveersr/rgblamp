use anyhow::ensure;

use crate::device::DeviceArgs;

#[derive(clap::Args, Debug)]
pub struct AutoCommand {
    /// Turn auto mode on/off
    #[arg(value_enum)]
    value: Switch,

    #[command(flatten)]
    device: DeviceArgs,
}

#[derive(clap::ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Switch {
    On,
    Off,
}

impl AutoCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        let mut devices = self.device.iter()?.peekable();
        let auto_mode = self.value == Switch::On;

        ensure!(devices.peek().is_some(), "no devices found");

        for device in devices {
            let (_, mut device) = device?;
            device.set_auto_mode(auto_mode)?;
        }

        Ok(())
    }
}
