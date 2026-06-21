use std::fs::File;

use crate::{
    LampArrayAttrs,
    error::LampResult,
    reports::{Report, ReportInfo},
    utils::{field::ReportField, io::get_feature, usage},
};

#[derive(Debug, Default)]
pub struct LampArrayAttrsReport {
    info: ReportInfo,

    lamp_count: ReportField,
    min_update_interval_us: ReportField,
}

impl LampArrayAttrsReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn get(&self, file: &mut File) -> LampResult<LampArrayAttrs> {
        let bytes = &get_feature(file, &self.info)?[1..];

        Ok(LampArrayAttrs {
            lamp_count: self.lamp_count.extract(bytes),
            min_update_interval_us: self.min_update_interval_us.extract(bytes),
        })
    }
}

impl Report for LampArrayAttrsReport {
    fn register(&mut self, usages: &[u16], size: u32) -> LampResult<()> {
        for usage in usages {
            let args = self.info.increment(size);
            match *usage {
                usage::LAMP_COUNT => self.lamp_count.set(args)?,
                usage::MIN_UPDATE_INTERVAL_US => self.min_update_interval_us.set(args)?,
                _ => (),
            }
        }

        Ok(())
    }

    fn validate(&self) -> LampResult<()> {
        self.info.validate()?;

        self.lamp_count.validate("LAMP_COUNT")?;
        self.min_update_interval_us
            .validate("MIN_UPDATE_INTERVAL_US")?;
        Ok(())
    }
}
