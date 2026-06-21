use std::fs::File;

use crate::{
    error::LampResult,
    reports::{Report, ReportInfo},
    utils::{field::ReportField, io::get_feature, usage},
};

#[derive(Debug, Default)]
pub struct LampAttrsResponseReport {
    info: ReportInfo,

    lamp_id: ReportField,
    update_latency_us: ReportField,
    is_programmable: ReportField<bool>,

    red_level_count: ReportField,
    green_level_count: ReportField,
    blue_level_count: ReportField,
    intensity_level_count: ReportField,
}

impl LampAttrsResponseReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn get(&self, file: &mut File) -> LampResult<LampAttrs> {
        let bytes = &get_feature(file, &self.info)?[1..];

        Ok(LampAttrs {
            lamp_id: self.lamp_id.get(bytes),
            update_latency_us: self.update_latency_us.get(bytes),
            is_programmable: self.is_programmable.get(bytes),

            red_level_count: self.red_level_count.get(bytes),
            green_level_count: self.green_level_count.get(bytes),
            blue_level_count: self.blue_level_count.get(bytes),
            intensity_level_count: self.intensity_level_count.get(bytes),
        })
    }
}

impl Report for LampAttrsResponseReport {
    fn info(&self) -> &ReportInfo {
        &self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) {
        for usage in usages {
            let field = self.info.create_field(size);
            match *usage {
                usage::LAMP_ID => self.lamp_id = field,
                usage::UPDATE_LATENCY_US => self.update_latency_us = field,
                usage::IS_PROGRAMMABLE => self.is_programmable = field.cast_as(),
                usage::RED_LEVEL_COUNT => self.red_level_count = field,
                usage::GREEN_LEVEL_COUNT => self.green_level_count = field,
                usage::BLUE_LEVEL_COUNT => self.blue_level_count = field,
                usage::INTENSITY_LEVEL_COUNT => self.intensity_level_count = field,
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct LampAttrs {
    pub lamp_id: u32,
    pub update_latency_us: u32,
    pub is_programmable: bool,

    pub red_level_count: u32,
    pub green_level_count: u32,
    pub blue_level_count: u32,
    pub intensity_level_count: u32,
}
