use std::{fs::File, ops::RangeInclusive};

use color::Rgba8;

use crate::{
    error::LampResult,
    reports::{LampUpdateFlags, Report, ReportInfo},
    utils::{
        field::ReportField,
        io::{prep_feature, set_feature},
        usage,
    },
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

    pub fn send(&self, file: &mut File, params: LampRangeUpdateParams) -> LampResult<()> {
        let LampRangeUpdateParams {
            lamp_ids,
            update_flags,
            color,
        } = params;

        let mut buffer = prep_feature(&self.info);
        let bytes = &mut buffer[1..];

        self.lamp_id_start.write(bytes, *lamp_ids.start());
        self.lamp_id_end.write(bytes, *lamp_ids.end());
        self.update_flags.write(bytes, update_flags);

        self.red.write(bytes, color.r);
        self.green.write(bytes, color.g);
        self.blue.write(bytes, color.b);
        self.intensity.write(bytes, color.a);

        set_feature(file, &mut buffer)
    }
}

impl Report for LampRangeUpdateReport {
    fn info(&self) -> &ReportInfo {
        &self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) -> LampResult<()> {
        for usage in usages {
            let args = self.info.increment(size);
            match *usage {
                usage::LAMP_ID_START => self.lamp_id_start.set(args)?,
                usage::LAMP_ID_END => self.lamp_id_end.set(args)?,
                usage::LAMP_UPDATE_FLAGS => self.update_flags.set(args)?,
                usage::RED_UPDATE_CHANNEL => self.red.set(args)?,
                usage::GREEN_UPDATE_CHANNEL => self.green.set(args)?,
                usage::BLUE_UPDATE_CHANNEL => self.blue.set(args)?,
                usage::INTENSITY_UPDATE_CHANNEL => self.intensity.set(args)?,
                _ => (),
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct LampRangeUpdateParams {
    pub lamp_ids: RangeInclusive<u32>,
    pub update_flags: LampUpdateFlags,
    pub color: Rgba8,
}
