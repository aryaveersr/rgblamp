use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
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

        for entry in fs::read_dir("/sys/class/hidraw").unwrap() {
            let entry_path = entry.unwrap().path();
            let descriptor = entry_path.clone().join("device/report_descriptor");
            let bytes = fs::read(descriptor).unwrap();

            if let Some(reports) = Reports::from_descriptor(&bytes) {
                let device_path = PathBuf::from("/dev").join(entry_path.file_name().unwrap());
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
        let array_attrs = reports.lamp_array_attrs.send(&mut file);
        let mut lamp_attrs = Vec::with_capacity(array_attrs.lamp_count as usize);

        assert!(array_attrs.lamp_count > 0);

        reports.lamp_attrs_request.send(&mut file, 0);
        for _ in 0..array_attrs.lamp_count {
            let attrs = reports.lamp_attrs_response.send(&mut file);
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

    pub fn set_auto_mode(&mut self, auto_mode: bool) {
        self.reports
            .lamp_array_control
            .send(&mut self.file, auto_mode);
    }
}
