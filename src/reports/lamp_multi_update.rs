use std::fs::File;

use crate::reports::{ReportField, ReportInfo};

#[derive(Debug, Default)]
pub struct LampMultiUpdateReport {
    pub(self) info: ReportInfo,
    pub(self) slots: u32,
    pub(self) lamp_count: ReportField,
    pub(self) lamp_update_flags: ReportField,
    pub(self) lamp_id_first: ReportField,
    pub(self) red_update_channel_first: ReportField,
    pub(self) green_update_channel_first: ReportField,
    pub(self) blue_update_channel_first: ReportField,
    pub(self) intensity_update_channel_first: ReportField,
}

impl LampMultiUpdateReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, params: LampMultiUpdateParams) {
        todo!()
    }
}

#[derive(Debug)]
pub struct LampMultiUpdateParams<'a> {
    pub lamp_update_flags: u16,
    pub items: &'a [LampMultiUpdateItem],
}

#[derive(Debug)]
pub struct LampMultiUpdateItem {
    pub lamp_id: u8,
    pub red_update_channel: u32,
    pub green_update_channel: u32,
    pub blue_update_channel: u32,
    pub intensity_update_channel: u32,
}
