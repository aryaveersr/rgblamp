use std::fs::File;

use crate::reports::{
    ReportField, ReportInfo,
    io::{prep_feature, set_feature},
};

#[derive(Debug, Default)]
pub struct LampAttributesRequestReport {
    pub(super) info: ReportInfo,
    pub(super) lamp_id: ReportField,
}

impl LampAttributesRequestReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, lamp_id: u8) {
        let mut bytes = prep_feature(&self.info);
        self.lamp_id.set(&mut bytes, lamp_id as u32);
        set_feature(file, &bytes);
    }
}
