use std::fs::File;

use crate::reports::{Report, ReportField, ReportInfo, consts, io::get_feature};

#[derive(Debug, Default)]
pub struct LampAttrsResponseReport {
    pub(self) info: ReportInfo,
    pub(self) lamp_id: ReportField,
    pub(self) update_latency_us: ReportField,
    pub(self) red_level_count: ReportField,
    pub(self) green_level_count: ReportField,
    pub(self) blue_level_count: ReportField,
    pub(self) intensity_level_count: ReportField,
    pub(self) is_programmable: ReportField<bool>,
}

impl LampAttrsResponseReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File) -> LampAttrs {
        let bytes = &get_feature(file, &self.info)[1..];

        LampAttrs {
            lamp_id: self.lamp_id.get(&bytes),
            update_latency_us: self.update_latency_us.get(&bytes),
            red_level_count: self.red_level_count.get(&bytes),
            green_level_count: self.green_level_count.get(&bytes),
            blue_level_count: self.blue_level_count.get(&bytes),
            intensity_level_count: self.intensity_level_count.get(&bytes),
            is_programmable: self.is_programmable.get(&bytes),
        }
    }
}

impl Report for LampAttrsResponseReport {
    fn get_info(&self) -> &ReportInfo {
        &self.info
    }

    fn get_info_mut(&mut self) -> &mut ReportInfo {
        &mut self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) {
        for usage in usages {
            let field = self.create_field(size);
            match *usage {
                consts::USAGE_LAMP_ID => self.lamp_id = field,
                consts::USAGE_UPDATE_LATENCY_US => self.update_latency_us = field,
                consts::USAGE_RED_LEVEL_COUNT => self.red_level_count = field,
                consts::USAGE_GREEN_LEVEL_COUNT => self.green_level_count = field,
                consts::USAGE_BLUE_LEVEL_COUNT => self.blue_level_count = field,
                consts::USAGE_INTENSITY_LEVEL_COUNT => self.intensity_level_count = field,
                consts::USAGE_IS_PROGRAMMABLE => self.is_programmable = field.cast_as(),
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct LampAttrs {
    pub lamp_id: u32,
    pub update_latency_us: u32,
    pub red_level_count: u32,
    pub green_level_count: u32,
    pub blue_level_count: u32,
    pub intensity_level_count: u32,
    pub is_programmable: bool,
}
