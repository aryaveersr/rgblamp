use std::fs::File;

use crate::reports::{
    Report, ReportField, ReportInfo, consts,
    io::{prep_feature, set_feature},
};

#[derive(Debug, Default)]
pub struct LampArrayControlReport {
    info: ReportInfo,
    auto_mode: ReportField<bool>,
}

impl Report for LampArrayControlReport {
    fn get_info(&self) -> &ReportInfo {
        &self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) {
        for usage in usages {
            let field = self.info.create_field(size);
            if *usage == consts::USAGE_AUTONOMOUS_MODE {
                self.auto_mode = field.cast_as();
            }
        }
    }
}

impl LampArrayControlReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, auto_mode: bool) {
        let mut buffer = prep_feature(&self.info);
        let bytes = &mut buffer[1..];
        self.auto_mode.set(bytes, auto_mode);
        set_feature(file, &mut buffer);
    }
}
