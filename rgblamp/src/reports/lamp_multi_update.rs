use std::fs::File;

use crate::{
    LampUpdateItem,
    reports::{LampUpdateFlags, Report, ReportInfo},
    utils::{
        field::ReportField,
        io::{prep_feature, set_feature},
        usage,
    },
};

#[derive(Debug, Default)]
pub struct LampMultiUpdateReport {
    info: ReportInfo,
    slots: u32,

    lamp_count: ReportField,
    update_flags: ReportField<LampUpdateFlags>,

    lamp_id: ReportField,
    red: ReportField<u8>,
    green: ReportField<u8>,
    blue: ReportField<u8>,
    intensity: ReportField<u8>,
}

impl LampMultiUpdateReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn slots(&self) -> u32 {
        self.slots
    }

    pub fn send(&self, file: &mut File, params: LampMultiUpdateParams) {
        let LampMultiUpdateParams {
            update_flags,
            items,
        } = params;

        assert!(items.len() <= self.slots as usize);

        let mut buffer = prep_feature(&self.info);
        let bytes = &mut buffer[1..];

        self.lamp_count.set(bytes, items.len() as u32);
        self.update_flags.set(bytes, update_flags);

        let mut lamp_id = self.lamp_id;

        let mut red = self.red;
        let mut green = self.green;
        let mut blue = self.blue;
        let mut intensity = self.intensity;

        let color_size = red.size * 4;

        for item in items {
            lamp_id.set(bytes, item.lamp_id);
            lamp_id += lamp_id.size;

            red.set(bytes, item.color.r);
            green.set(bytes, item.color.g);
            blue.set(bytes, item.color.b);
            intensity.set(bytes, item.color.a);

            red += color_size;
            green += color_size;
            blue += color_size;
            intensity += color_size;
        }

        set_feature(file, &mut buffer);
    }
}

impl Report for LampMultiUpdateReport {
    fn info(&self) -> &ReportInfo {
        &self.info
    }

    fn register(&mut self, usages: &[u16], size: u32) {
        for usage in usages {
            let field = self.info.create_field(size);
            match *usage {
                usage::LAMP_COUNT => self.lamp_count = field,
                usage::LAMP_UPDATE_FLAGS => self.update_flags = field.cast_as(),
                usage::LAMP_ID => {
                    self.slots += 1;
                    if self.slots == 1 {
                        self.lamp_id = field.cast_as();
                    }
                }
                usage::RED_UPDATE_CHANNEL if self.red.is_uninit() => {
                    self.red = field.cast_as();
                }
                usage::GREEN_UPDATE_CHANNEL if self.green.is_uninit() => {
                    self.green = field.cast_as();
                }
                usage::BLUE_UPDATE_CHANNEL if self.blue.is_uninit() => {
                    self.blue = field.cast_as();
                }
                usage::INTENSITY_UPDATE_CHANNEL if self.intensity.is_uninit() => {
                    self.intensity = field.cast_as();
                }
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct LampMultiUpdateParams<'a> {
    pub update_flags: LampUpdateFlags,
    pub items: &'a [LampUpdateItem],
}
