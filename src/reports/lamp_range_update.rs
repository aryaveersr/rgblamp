use std::fs::File;

use crate::reports::{
    Report, ReportField, ReportInfo,
    io::{prep_feature, set_feature},
};

#[derive(Debug, Default)]
pub struct LampRangeUpdateReport {
    pub(self) info: ReportInfo,
    pub(self) lamp_id_start: ReportField,
    pub(self) lamp_id_end: ReportField,
    pub(self) lamp_update_flags: ReportField<u16>,
    pub(self) red_update_channel: ReportField,
    pub(self) green_update_channel: ReportField,
    pub(self) blue_update_channel: ReportField,
    pub(self) intensity_update_channel: ReportField,
}

impl LampRangeUpdateReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, params: LampRangeUpdateParams) {
        let mut bytes = prep_feature(&self.info);

        self.lamp_id_start.set(&mut bytes, params.lamp_id_start);
        self.lamp_id_end.set(&mut bytes, params.lamp_id_end);
        self.lamp_update_flags
            .set(&mut bytes, params.lamp_update_flags);
        self.red_update_channel
            .set(&mut bytes, params.red_update_channel);
        self.green_update_channel
            .set(&mut bytes, params.green_update_channel);
        self.blue_update_channel
            .set(&mut bytes, params.blue_update_channel);
        self.intensity_update_channel
            .set(&mut bytes, params.intensity_update_channel);

        set_feature(file, &bytes);
    }
}

impl Report for LampRangeUpdateReport {
    fn get_info(&self) -> &ReportInfo {
        &self.info
    }

    fn get_info_mut(&mut self) -> &mut ReportInfo {
        &mut self.info
    }
}

#[derive(Debug)]
pub struct LampRangeUpdateParams {
    pub lamp_id_start: u32,
    pub lamp_id_end: u32,
    pub lamp_update_flags: u16,
    pub red_update_channel: u32,
    pub green_update_channel: u32,
    pub blue_update_channel: u32,
    pub intensity_update_channel: u32,
}
