use anyhow::ensure;

use crate::{device::DeviceArgs, effects::rainbow::Rainbow};

#[derive(clap::Args, Debug)]
pub struct EffectsCommand {
    #[arg(value_enum)]
    effect: Effect,

    #[command(flatten)]
    device: DeviceArgs,
}

#[derive(clap::ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Effect {
    Rainbow,
}

impl EffectsCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        let devices = self
            .device
            .iter()?
            .map(|d| d.map(|(_, d)| d))
            .collect::<anyhow::Result<Vec<_>>>()?;

        ensure!(!devices.is_empty(), "no devices found");

        match self.effect {
            Effect::Rainbow => Rainbow::new(devices).run()?,
        }

        Ok(())
    }
}
