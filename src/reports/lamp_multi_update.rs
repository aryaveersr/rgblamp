use std::fs::File;

use crate::reports::{
    Report, ReportField, ReportInfo, UpdateFlags, consts,
    io::{prep_feature, set_feature},
};

#[derive(Debug, Default)]
pub struct LampMultiUpdateReport {
    info: ReportInfo,
    lamp_count: ReportField,
    lamp_update_flags: ReportField<UpdateFlags>,
    slots: u32,
    lamp_id_first: ReportField,
    red_update_channel_first: ReportField,
    green_update_channel_first: ReportField,
    blue_update_channel_first: ReportField,
    intensity_update_channel_first: ReportField,
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
        assert!(params.items.len() <= self.slots as usize);
        assert!(params.items.len() <= u32::MAX as usize);

        let mut bytes = prep_feature(&self.info);

        self.lamp_count.set(&mut bytes, params.items.len() as u32);
        self.lamp_update_flags
            .set(&mut bytes, params.lamp_update_flags);

        let mut lamp_id = self.lamp_id_first.clone();
        let mut red_channel = self.red_update_channel_first.clone();
        let mut green_channel = self.green_update_channel_first.clone();
        let mut blue_channel = self.blue_update_channel_first.clone();
        let mut intensity_channel = self.intensity_update_channel_first.clone();

        let color_size = red_channel.size * 4;

        for item in params.items {
            lamp_id.set(&mut bytes, item.lamp_id);
            red_channel.set(&mut bytes, item.red_update_channel);
            green_channel.set(&mut bytes, item.green_update_channel);
            blue_channel.set(&mut bytes, item.blue_update_channel);
            intensity_channel.set(&mut bytes, item.intensity_update_channel);

            lamp_id.offset += lamp_id.size;
            red_channel.offset += color_size;
            green_channel.offset += color_size;
            blue_channel.offset += color_size;
            intensity_channel.offset += color_size;
        }

        set_feature(file, &bytes);
    }
}

impl Report for LampMultiUpdateReport {
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
                consts::USAGE_LAMP_COUNT => self.lamp_count = field,
                consts::USAGE_LAMP_UPDATE_FLAGS => self.lamp_update_flags = field.cast_as(),
                consts::USAGE_LAMP_ID => {
                    if self.slots == 0 {
                        self.lamp_id_first = field.cast_as();
                    }
                    self.slots += 1;
                }
                consts::USAGE_RED_UPDATE_CHANNEL if self.slots == 1 => {
                    self.red_update_channel_first = field
                }
                consts::USAGE_GREEN_UPDATE_CHANNEL if self.slots == 1 => {
                    self.green_update_channel_first = field
                }
                consts::USAGE_BLUE_UPDATE_CHANNEL if self.slots == 1 => {
                    self.blue_update_channel_first = field
                }
                consts::USAGE_INTENSITY_UPDATE_CHANNEL if self.slots == 1 => {
                    self.intensity_update_channel_first = field
                }
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct LampMultiUpdateParams<'a> {
    pub lamp_update_flags: UpdateFlags,
    pub items: &'a [LampMultiUpdateItem],
}

#[derive(Debug)]
pub struct LampMultiUpdateItem {
    pub lamp_id: u32,
    pub red_update_channel: u32,
    pub green_update_channel: u32,
    pub blue_update_channel: u32,
    pub intensity_update_channel: u32,
}
