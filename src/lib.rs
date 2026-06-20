use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

use crate::reports::Reports;

mod reports;

#[derive(Debug)]
pub struct LampArray {
    file: File,
    reports: Reports,
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

                let lamparray = Self { file, reports };
                lamparrays.push(lamparray);
            }
        }

        lamparrays
    }
}
