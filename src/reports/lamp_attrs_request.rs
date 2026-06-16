use std::fs::File;

use crate::reports::{
    Report, ReportField, ReportInfo, consts,
    io::{prep_feature, set_feature},
};

#[derive(Debug, Default)]
pub struct LampAttrsRequestReport {
    info: ReportInfo,
    lamp_id: ReportField,
}

impl Report for LampAttrsRequestReport {
    fn get_info(&self) -> &ReportInfo {
        &self.info
    }

    fn get_info_mut(&mut self) -> &mut ReportInfo {
        &mut self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) {
        for usage in usages {
            let field = self.create_field(size);
            if *usage == consts::USAGE_LAMP_ID {
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
        let mut bytes = prep_feature(&self.info);
        self.lamp_id.set(&mut bytes, lamp_id);
        set_feature(file, &bytes);
    }
}
