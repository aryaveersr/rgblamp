use std::fs::File;

use crate::reports::{ReportField, ReportInfo, io::get_feature};

#[derive(Debug, Default)]
pub struct LampArrayAttributesReport {
    pub(super) info: ReportInfo,
    pub(super) lamp_count: ReportField,
    pub(super) min_update_interval_us: ReportField,
}

impl LampArrayAttributesReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File) -> LampArrayAttributes {
        let bytes = &get_feature(file, &self.info)[1..];

        LampArrayAttributes {
            lamp_count: self.lamp_count.get(bytes),
            min_update_interval_us: self.min_update_interval_us.get(bytes),
        }
    }
}

#[derive(Debug)]
pub struct LampArrayAttributes {
    pub lamp_count: u32,
    pub min_update_interval_us: u32,
}
