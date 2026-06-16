use std::fs::File;

use crate::reports::{ReportField, ReportInfo, io::get_feature};

#[derive(Debug, Default)]
pub struct LampAttributesResponseReport {
    pub(self) info: ReportInfo,
    pub(self) lamp_id: ReportField,
    pub(self) update_latency_us: ReportField,
    pub(self) red_level_count: ReportField,
    pub(self) green_level_count: ReportField,
    pub(self) blue_level_count: ReportField,
    pub(self) intensity_level_count: ReportField,
    pub(self) is_programmable: ReportField,
}

impl LampAttributesResponseReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File) -> LampAttributes {
        let bytes = &get_feature(file, &self.info)[1..];

        LampAttributes {
            lamp_id: self.lamp_id.get(&bytes) as u8,
            update_latency_us: self.update_latency_us.get(&bytes),
            red_level_count: self.red_level_count.get(&bytes),
            green_level_count: self.green_level_count.get(&bytes),
            blue_level_count: self.blue_level_count.get(&bytes),
            intensity_level_count: self.intensity_level_count.get(&bytes),
            is_programmable: self.is_programmable.get(&bytes) != 0,
        }
    }
}

#[derive(Debug)]
pub struct LampAttributes {
    pub lamp_id: u8,
    pub update_latency_us: u32,
    pub red_level_count: u32,
    pub green_level_count: u32,
    pub blue_level_count: u32,
    pub intensity_level_count: u32,
    pub is_programmable: bool,
}
