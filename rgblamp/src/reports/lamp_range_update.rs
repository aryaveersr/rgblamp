use std::fs::File;

use crate::reports::{
    LampUpdateFlags, Report, ReportField, ReportInfo, consts,
    io::{prep_feature, set_feature},
};

#[derive(Debug, Default)]
pub struct LampRangeUpdateReport {
    info: ReportInfo,
    lamp_id_start: ReportField,
    lamp_id_end: ReportField,
    lamp_update_flags: ReportField<LampUpdateFlags>,
    red_update_channel: ReportField,
    green_update_channel: ReportField,
    blue_update_channel: ReportField,
    intensity_update_channel: ReportField,
}

impl LampRangeUpdateReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, params: LampRangeUpdateParams) {
        let mut buffer = prep_feature(&self.info);
        let bytes = &mut buffer[1..];

        self.lamp_id_start.set(bytes, params.lamp_id_start);
        self.lamp_id_end.set(bytes, params.lamp_id_end);
        self.lamp_update_flags.set(bytes, params.lamp_update_flags);
        self.red_update_channel
            .set(bytes, params.red_update_channel);
        self.green_update_channel
            .set(bytes, params.green_update_channel);
        self.blue_update_channel
            .set(bytes, params.blue_update_channel);
        self.intensity_update_channel
            .set(bytes, params.intensity_update_channel);

        set_feature(file, &mut buffer);
    }
}

impl Report for LampRangeUpdateReport {
    fn get_info(&self) -> &ReportInfo {
        &self.info
    }

    fn get_info_mut(&mut self) -> &mut ReportInfo {
        &mut self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) {
        for usage in usages {
            let field = self.create_field(size);
            match *usage {
                consts::USAGE_LAMP_ID_START => self.lamp_id_start = field,
                consts::USAGE_LAMP_ID_END => self.lamp_id_end = field,
                consts::USAGE_LAMP_UPDATE_FLAGS => self.lamp_update_flags = field.cast_as(),
                consts::USAGE_RED_UPDATE_CHANNEL => self.red_update_channel = field,
                consts::USAGE_GREEN_UPDATE_CHANNEL => self.green_update_channel = field,
                consts::USAGE_BLUE_UPDATE_CHANNEL => self.blue_update_channel = field,
                consts::USAGE_INTENSITY_UPDATE_CHANNEL => self.intensity_update_channel = field,
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct LampRangeUpdateParams {
    pub lamp_id_start: u32,
    pub lamp_id_end: u32,
    pub lamp_update_flags: LampUpdateFlags,
    pub red_update_channel: u32,
    pub green_update_channel: u32,
    pub blue_update_channel: u32,
    pub intensity_update_channel: u32,
}
