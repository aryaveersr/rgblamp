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
            lamp_attrs.push(attrs);
        }

        Self {
            file,
            reports,
            array_attrs,
            lamp_attrs,
        }
    }

    pub fn attrs(&self) -> &LampArrayAttrs {
        &self.array_attrs
    }

    pub fn min_update_interval(&self) -> Duration {
        Duration::from_micros(self.array_attrs.min_update_interval_us as u64)
    }

    pub fn lamp_attrs(&self) -> &[LampAttrs] {
        &self.lamp_attrs
    }

    pub fn set_auto_mode(&mut self, auto_mode: bool) {
        self.reports
            .lamp_array_control
            .send(&mut self.file, auto_mode);
    }

    pub fn set_color_all<T: ColorSpace>(&mut self, color: AlphaColor<T>) {
        let color = color.to_rgba8();

        self.reports.lamp_range_update.send(
            &mut self.file,
            LampRangeUpdateParams {
                lamp_id_start: 0,
                lamp_id_end: self.array_attrs.lamp_count - 1,
                red_update_channel: color.r as u32,
                green_update_channel: color.g as u32,
                blue_update_channel: color.b as u32,
                intensity_update_channel: color.a as u32,
                lamp_update_flags: LampUpdateFlags::new(true),
            },
        );
    }

    pub fn set_color_lamp<T: ColorSpace>(&mut self, lamp_id: u32, color: AlphaColor<T>) {
        let color = color.to_rgba8();

        self.reports.lamp_range_update.send(
            &mut self.file,
            LampRangeUpdateParams {
                lamp_id_start: lamp_id,
                lamp_id_end: lamp_id,
                red_update_channel: color.r as u32,
                green_update_channel: color.g as u32,
                blue_update_channel: color.b as u32,
                intensity_update_channel: color.a as u32,
                lamp_update_flags: LampUpdateFlags::new(true),
            },
        );
    }
}
