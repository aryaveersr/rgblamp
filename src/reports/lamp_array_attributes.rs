use std::fs::File;

use crate::reports::{Report, ReportField, ReportInfo, io::get_feature};

#[derive(Debug, Default)]
pub struct LampArrayAttributesReport {
    pub(super) info: ReportInfo,
    pub(super) lamp_count: ReportField,
    pub(super) min_update_interval_us: ReportField,
}

impl LampArrayAttributesReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File) -> LampArrayAttributes {
        let bytes = &get_feature(file, &self.info)[1..];

        LampArrayAttributes {
            lamp_count: self.lamp_count.get(bytes),
            min_update_interval_us: self.min_update_interval_us.get(bytes),
        }
    }
}

impl Report for LampArrayAttributesReport {
    fn get_info(&self) -> &ReportInfo {
        &self.info
    }

    fn get_info_mut(&mut self) -> &mut ReportInfo {
        &mut self.info
    }
}

#[derive(Debug)]
pub struct LampArrayAttributes {
    pub lamp_count: u32,
    pub min_update_interval_us: u32,
}

//
// ReportKind::LampArrayAttributes
//     if let Some(report) = self.lamp_array_attributes_report.as_mut() =>
// {
//     match usage {
//         USAGE_LAMP_COUNT => report.lamp_count = field,
//         USAGE_MIN_UPDATE_INTERVAL_US => report.min_update_interval_us = field,
//         _ => (),
//     }
// }
