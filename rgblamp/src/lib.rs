use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
    time::Duration,
};

use color::{AlphaColor, ColorSpace};

use crate::reports::{
    LampUpdateFlags, Reports, lamp_array_attrs::LampArrayAttrs, lamp_attrs_response::LampAttrs,
    lamp_range_update::LampRangeUpdateParams,
};

mod reports;

#[derive(Debug)]
pub struct LampArray {
    file: File,
    reports: Reports,
    array_attrs: LampArrayAttrs,
    lamp_attrs: Vec<LampAttrs>,
}

impl LampArray {
    pub fn enumerate() -> Vec<Self> {
        let mut lamparrays = Vec::new();
        let mut entries: Vec<_> = fs::read_dir("/sys/class/hidraw")
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();

        entries.sort_by_key(|dir| dir.file_name());

        for entry in entries {
            let descriptor = entry.path().join("device/report_descriptor");
            let bytes = fs::read(descriptor).unwrap();

            if let Some(reports) = Reports::from_descriptor(&bytes) {
                let device_path = PathBuf::from("/dev").join(entry.path().file_name().unwrap());
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(device_path)
                    .unwrap();

                lamparrays.push(Self::new(file, reports));
            }
        }

        lamparrays
    }

    fn new(mut file: File, reports: Reports) -> Self {
        let array_attrs = reports.lamp_array_attrs.get(&mut file);
        let mut lamp_attrs = Vec::with_capacity(array_attrs.lamp_count as usize);

        assert!(array_attrs.lamp_count > 0);

        reports.lamp_attrs_request.send(&mut file, 0);
        for _ in 0..array_attrs.lamp_count {
            let attrs = reports.lamp_attrs_response.get(&mut file);

            assert!(
                attrs.is_programmable,
                "Non-programmable lamps are not supported (TODO)"
            );

            lamp_attrs.push(attrs);
        }

        Self {
            file,
            reports,
            array_attrs,
            lamp_attrs,
        }
    }

    pub fn lamp_count(&self) -> u32 {
        self.array_attrs.lamp_count
    }

    pub fn min_update_interval(&self) -> Duration {
        Duration::from_micros(self.array_attrs.min_update_interval_us as u64)
    }

    pub fn set_auto_mode(&mut self, auto_mode: bool) {
        self.reports
            .lamp_array_control
            .send(&mut self.file, auto_mode);
    }

    pub fn set_lamp<T: ColorSpace>(&mut self, lamp_id: u32, color: AlphaColor<T>) {
        let color = color.to_rgba8();

        self.reports.lamp_range_update.send(
            &mut self.file,
            LampRangeUpdateParams {
                lamp_ids: lamp_id..=lamp_id,
                update_flags: LampUpdateFlags::new(true),
                color,
            },
        );
    }

    pub fn set_all_lamps<T: ColorSpace>(&mut self, color: AlphaColor<T>) {
        let color = color.to_rgba8();

        self.reports.lamp_range_update.send(
            &mut self.file,
            LampRangeUpdateParams {
                lamp_ids: 0..=(self.array_attrs.lamp_count - 1),
                update_flags: LampUpdateFlags::new(true),
                color,
            },
        );
    }
}
