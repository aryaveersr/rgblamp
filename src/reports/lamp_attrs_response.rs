use std::fs::File;

use crate::{
    LampAttrs,
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
            lamp_id: self.lamp_id.extract(bytes),
            update_latency_us: self.update_latency_us.extract(bytes),
            is_programmable: self.is_programmable.extract(bytes),

            red_level_count: self.red_level_count.extract(bytes),
            green_level_count: self.green_level_count.extract(bytes),
            blue_level_count: self.blue_level_count.extract(bytes),
            intensity_level_count: self.intensity_level_count.extract(bytes),
        })
    }
}

impl Report for LampAttrsResponseReport {
    fn register(&mut self, usages: &[u16], size: u32) -> LampResult<()> {
        for usage in usages {
            let args = self.info.increment(size);
            match *usage {
                usage::LAMP_ID => self.lamp_id.set(args)?,
                usage::UPDATE_LATENCY_US => self.update_latency_us.set(args)?,
                usage::IS_PROGRAMMABLE => self.is_programmable.set(args)?,
                usage::RED_LEVEL_COUNT => self.red_level_count.set(args)?,
                usage::GREEN_LEVEL_COUNT => self.green_level_count.set(args)?,
                usage::BLUE_LEVEL_COUNT => self.blue_level_count.set(args)?,
                usage::INTENSITY_LEVEL_COUNT => self.intensity_level_count.set(args)?,
                _ => (),
            }
        }

        Ok(())
    }

    fn validate(&self) -> LampResult<()> {
        self.info.validate()?;

        self.lamp_id.validate("LAMP_ID")?;
        self.update_latency_us.validate("UPDATE_LATENCY_US")?;
        self.is_programmable.validate("IS_PROGRAMMABLE")?;
        self.red_level_count.validate("RED_LEVEL_COUNT")?;
        self.green_level_count.validate("GREEN_LEVEL_COUNT")?;
        self.blue_level_count.validate("BLUE_LEVEL_COUNT")?;
        self.intensity_level_count
            .validate("INTENSITY_LEVEL_COUNT")?;
        Ok(())
    }
}
