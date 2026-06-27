use color::palette::css;

use crate::device::DeviceArgs;

#[derive(clap::Args, Debug)]
pub struct EffectCommand {
    #[command(subcommand)]
    effect: Effect,
}

#[derive(clap::Subcommand, Debug)]
pub enum Effect {
    Rainbow(Rainbow),
}

impl EffectCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        match &self.effect {
            Effect::Rainbow(rainbow) => rainbow.run(),
        }
    }
}

#[derive(clap::Args, Debug)]
pub struct Rainbow {
    #[command(flatten)]
    device: DeviceArgs,
}

impl Rainbow {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut devices = self
            .device
            .iter()?
            .map(|d| d.map(|(_, d)| d))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let sleep_duration = devices
            .iter()
            .map(|d| d.min_update_interval())
            .max()
            .unwrap();

        for device in &mut devices {
            device.set_auto_mode(false)?;
        }

        let mut color = css::BLUE;

        loop {
            for device in &mut devices {
                device.set_all_lamps(color.to_rgba8())?;
            }

            color = color.map_hue(|mut hue| {
                hue += 2.0;
                if hue >= 360.0 {
                    hue = 0.0;
                }
                hue
            });

            std::thread::sleep(sleep_duration);
        }
    }
}
