use color::palette::css;
use rgblamp::LampArray;

#[derive(Debug)]
pub struct Rainbow {
    devices: Vec<LampArray>,
}

impl Rainbow {
    pub fn new(devices: Vec<LampArray>) -> Self {
        Self { devices }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let sleep_duration = self
            .devices
            .iter()
            .map(|d| d.min_update_interval())
            .max()
            .unwrap();

        for device in &mut self.devices {
            device.set_auto_mode(false)?;
        }

        let mut color = css::BLUE;

        loop {
            for device in &mut self.devices {
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
