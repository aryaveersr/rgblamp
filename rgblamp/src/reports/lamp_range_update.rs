use std::{fs::File, ops::RangeInclusive};

use color::Rgba8;

use crate::reports::{
    LampUpdateFlags, Report, ReportField, ReportInfo,
    io::{prep_feature, set_feature},
    usage,
};

#[derive(Debug, Default)]
pub struct LampRangeUpdateReport {
    info: ReportInfo,

    lamp_id_start: ReportField,
    lamp_id_end: ReportField,
    update_flags: ReportField<LampUpdateFlags>,

    red: ReportField<u8>,
    green: ReportField<u8>,
    blue: ReportField<u8>,
    intensity: ReportField<u8>,
}

impl LampRangeUpdateReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, params: LampRangeUpdateParams) {
        let LampRangeUpdateParams {
            lamp_ids,
            update_flags,
            color,
        } = params;

        let mut buffer = prep_feature(&self.info);
        let bytes = &mut buffer[1..];

        self.lamp_id_start.set(bytes, *lamp_ids.start());
        self.lamp_id_end.set(bytes, *lamp_ids.end());
        self.update_flags.set(bytes, update_flags);

        self.red.set(bytes, color.r);
        self.green.set(bytes, color.g);
        self.blue.set(bytes, color.b);
        self.intensity.set(bytes, color.a);

        set_feature(file, &mut buffer);
    }
}

impl Report for LampRangeUpdateReport {
    fn info(&self) -> &ReportInfo {
        &self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) {
        for usage in usages {
            let field = self.info.create_field(size);
            match *usage {
                usage::LAMP_ID_START => self.lamp_id_start = field,
                usage::LAMP_ID_END => self.lamp_id_end = field,
                usage::LAMP_UPDATE_FLAGS => self.update_flags = field.cast_as(),
                usage::RED_UPDATE_CHANNEL => self.red = field.cast_as(),
                usage::GREEN_UPDATE_CHANNEL => self.green = field.cast_as(),
                usage::BLUE_UPDATE_CHANNEL => self.blue = field.cast_as(),
                usage::INTENSITY_UPDATE_CHANNEL => self.intensity = field.cast_as(),
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct LampRangeUpdateParams {
    pub lamp_ids: RangeInclusive<u32>,
    pub update_flags: LampUpdateFlags,
    pub color: Rgba8,
}
