use std::fs::File;

use crate::reports::{
    Report, ReportField, ReportInfo,
    io::{prep_feature, set_feature},
    usage,
};

#[derive(Debug, Default)]
pub struct LampAttrsRequestReport {
    info: ReportInfo,

    lamp_id: ReportField,
}

impl Report for LampAttrsRequestReport {
    fn info(&self) -> &ReportInfo {
        &self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) {
        for usage in usages {
            let field = self.info.create_field(size);
            if *usage == usage::LAMP_ID {
                self.lamp_id = field;
            }
        }
    }
}

impl LampAttrsRequestReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, lamp_id: u32) {
        let mut buffer = prep_feature(&self.info);
        let bytes = &mut buffer[1..];
        self.lamp_id.set(bytes, lamp_id);
        set_feature(file, &mut buffer);
    }
}
