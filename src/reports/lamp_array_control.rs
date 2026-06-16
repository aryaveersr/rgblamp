use std::fs::File;

use crate::reports::{
    Report, ReportField, ReportInfo,
    io::{prep_feature, set_feature},
};

#[derive(Debug, Default)]
pub struct LampArrayControlReport {
    pub(super) info: ReportInfo,
    pub(super) auto_mode: ReportField<bool>,
}

impl Report for LampArrayControlReport {
    fn get_info(&self) -> &ReportInfo {
        &self.info
    }

    fn get_info_mut(&mut self) -> &mut ReportInfo {
        &mut self.info
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
        let mut bytes = prep_feature(&self.info);
        self.auto_mode.set(&mut bytes, auto_mode);
        set_feature(file, &bytes);
    }
}
