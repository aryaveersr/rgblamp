use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

use crate::reports::{Reports, lamp_array_attrs::LampArrayAttrs, lamp_attrs_response::LampAttrs};

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

        if array_attrs.lamp_count > 0 {
            reports.lamp_attrs_request.send(&mut file, 0);
            for _ in 0..array_attrs.lamp_count {
                let attrs = reports.lamp_attrs_response.send(&mut file);
                lamp_attrs.push(attrs);
            }
        }

        Self {
            file,
            reports,
            array_attrs,
            lamp_attrs,
        }
    }
}
