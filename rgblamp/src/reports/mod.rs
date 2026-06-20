//! References:
//!
//! HID Spec: https://www.usb.org/document-library/device-class-definition-hid-111
//! HUT:      https://usb.org/document-library/hid-usage-tables-14
//!
//! The HUT (HID Usage Tables) document has the information for the LampArray interface
//! under Section 26: Lighting and Illumination Page.

use std::fmt::Debug;

use bilge::prelude::*;

use crate::reports::{
    field::ReportField, lamp_array_attrs::LampArrayAttrsReport,
    lamp_array_control::LampArrayControlReport, lamp_attrs_request::LampAttrsRequestReport,
    lamp_attrs_response::LampAttrsResponseReport, lamp_multi_update::LampMultiUpdateReport,
    lamp_range_update::LampRangeUpdateReport, parser::ReportDescriptorParser,
};

pub mod lamp_array_attrs;
pub mod lamp_array_control;
pub mod lamp_attrs_request;
pub mod lamp_attrs_response;
pub mod lamp_multi_update;
pub mod lamp_range_update;

mod field;
mod io;
mod parser;

#[derive(Debug)]
pub struct Reports {
    pub lamp_array_attrs: LampArrayAttrsReport,
    pub lamp_attrs_request: LampAttrsRequestReport,
    pub lamp_attrs_response: LampAttrsResponseReport,
    pub lamp_multi_update: LampMultiUpdateReport,
    pub lamp_range_update: LampRangeUpdateReport,
    pub lamp_array_control: LampArrayControlReport,
}

impl Reports {
    pub fn from_descriptor(bytes: &[u8]) -> Option<Self> {
        ReportDescriptorParser::new(bytes).parse()
    }
}

#[derive(Debug, Default)]
struct ReportInfo {
    pub id: u8,
    pub size: u32,
}

impl ReportInfo {
    pub fn new(id: u8) -> Self {
        Self { id, size: 0 }
    }

    pub fn validate(&self) {
        assert_eq!(self.size % 8, 0)
    }

    pub fn bytes_len(&self) -> usize {
        self.size as usize / 8
    }

    fn create_field(&mut self, size: u32) -> ReportField {
        let field = ReportField::new(self.size, size);
        self.size += size;
        field
    }
}

trait Report {
    fn info(&self) -> &ReportInfo;
    fn register(&mut self, usages: &[u16], size: u32);
}

#[bitsize(16)]
#[derive(FromBits, DebugBits, DefaultBits)]
pub struct LampUpdateFlags {
    pub complete: bool,
    _reserved: u15,
}

impl TryFrom<u32> for LampUpdateFlags {
    type Error = <u16 as TryFrom<u32>>::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(u16::try_from(value)?.into())
    }
}

impl From<LampUpdateFlags> for u32 {
    fn from(value: LampUpdateFlags) -> Self {
        u16::from(value) as u32
    }
}

mod consts {
    pub const USAGE_PAGE_LIGHTING: u16 = 0x59;

    pub const USAGE_LAMP_ARRAY: u16 = 0x1;
    pub const USAGE_LAMP_ARRAY_ATTRIBUTES_REPORT: u16 = 0x2;
    pub const USAGE_LAMP_COUNT: u16 = 0x3;
    #[expect(unused)]
    pub const USAGE_LAMP_ARRAY_KIND: u16 = 0x7;
    pub const USAGE_MIN_UPDATE_INTERVAL_US: u16 = 0x8;
    pub const USAGE_LAMP_ATTRIBUTES_REQUEST_REPORT: u16 = 0x20;
    pub const USAGE_LAMP_ID: u16 = 0x21;
    pub const USAGE_LAMP_ATTRIBUTES_RESPONSE_REPORT: u16 = 0x22;
    pub const USAGE_UPDATE_LATENCY_US: u16 = 0x27;
    pub const USAGE_RED_LEVEL_COUNT: u16 = 0x28;
    pub const USAGE_GREEN_LEVEL_COUNT: u16 = 0x29;
    pub const USAGE_BLUE_LEVEL_COUNT: u16 = 0x2A;
    pub const USAGE_INTENSITY_LEVEL_COUNT: u16 = 0x2B;
    pub const USAGE_IS_PROGRAMMABLE: u16 = 0x2C;
    pub const USAGE_LAMP_MULTI_UPDATE_REPORT: u16 = 0x50;
    pub const USAGE_RED_UPDATE_CHANNEL: u16 = 0x51;
    pub const USAGE_GREEN_UPDATE_CHANNEL: u16 = 0x52;
    pub const USAGE_BLUE_UPDATE_CHANNEL: u16 = 0x53;
    pub const USAGE_INTENSITY_UPDATE_CHANNEL: u16 = 0x54;
    pub const USAGE_LAMP_UPDATE_FLAGS: u16 = 0x55;
    pub const USAGE_LAMP_RANGE_UPDATE_REPORT: u16 = 0x60;
    pub const USAGE_LAMP_ID_START: u16 = 0x61;
    pub const USAGE_LAMP_ID_END: u16 = 0x62;
    pub const USAGE_LAMP_ARRAY_CONTROL_REPORT: u16 = 0x70;
    pub const USAGE_AUTONOMOUS_MODE: u16 = 0x71;
}
