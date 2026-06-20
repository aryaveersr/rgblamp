use std::fs::File;

use crate::reports::{Report, ReportField, ReportInfo, consts, io::get_feature};

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

    pub fn send(&self, file: &mut File) -> LampArrayAttrs {
        let bytes = &get_feature(file, &self.info)[1..];

        LampArrayAttrs {
            lamp_count: self.lamp_count.get(bytes),
            min_update_interval_us: self.min_update_interval_us.get(bytes),
        }
    }
}

impl Report for LampArrayAttrsReport {
    fn info(&self) -> &ReportInfo {
        &self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) {
        for usage in usages {
            let field = self.info.create_field(size);
            match *usage {
                consts::USAGE_LAMP_COUNT => self.lamp_count = field,
                consts::USAGE_MIN_UPDATE_INTERVAL_US => self.min_update_interval_us = field,
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct LampArrayAttrs {
    pub lamp_count: u32,
    pub min_update_interval_us: u32,
}
