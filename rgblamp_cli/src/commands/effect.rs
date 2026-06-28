use std::time::Duration;

use color::palette::css;
use log::error;
use rgblamp::LampArray;

use crate::device::{DeviceArgs, HidInfo};

#[derive(clap::Args, Debug)]
pub struct EffectCommand {
    #[command(subcommand)]
    effect: Effect,

    #[command(flatten)]
    device: DeviceArgs,
}

#[derive(clap::Subcommand, Debug)]
pub enum Effect {
    Rainbow(Rainbow),
}

impl EffectCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        let devices = self.device.enumerate()?;

        let sleep_duration = devices
            .iter()
            .map(|d| d.1.min_update_interval())
            .max()
            .unwrap();

        match &self.effect {
            Effect::Rainbow(rainbow) => rainbow.run(devices, sleep_duration),
        }
    }
}

#[derive(clap::Args, Debug)]
pub struct Rainbow {
    /// Speed. Negative values reverse the effect. Can be fractional
    #[arg(short, long, default_value_t = 1.0)]
    speed: f32,

    /// Retry if IO fails. Recommended in case of frequent crashes
    #[arg(short, long)]
    retry: bool,
}

impl Rainbow {
    pub fn run(
        &self,
        mut devices: Vec<(HidInfo, LampArray)>,
        sleep_duration: Duration,
    ) -> anyhow::Result<()> {
        for (_, device) in &mut devices {
            device.set_auto_mode(false)?;
        }

        let mut color = css::BLUE;

        loop {
            for (_, device) in &mut devices {
                if let Err(err) = device.set_all_lamps(color.to_rgba8()) {
                    if self.retry {
                        error!("{err}");
                    } else {
                        Err(err)?;
                    }
                }
            }

            color = color.map_hue(|mut hue| {
                hue += self.speed;
                if hue >= 360.0 {
                    hue = 0.0;
                }
                hue
            });

            std::thread::sleep(sleep_duration);
        }
    }
}
