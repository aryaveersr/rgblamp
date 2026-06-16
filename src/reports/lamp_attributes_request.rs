use std::fs::File;

use crate::reports::{
    Report, ReportField, ReportInfo,
    io::{prep_feature, set_feature},
};

#[derive(Debug, Default)]
pub struct LampAttributesRequestReport {
    pub(super) info: ReportInfo,
    pub(super) lamp_id: ReportField,
}

impl Report for LampAttributesRequestReport {
    fn get_info(&self) -> &ReportInfo {
        &self.info
    }

    fn get_info_mut(&mut self) -> &mut ReportInfo {
        &mut self.info
    }
}

impl LampAttributesRequestReport {
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
