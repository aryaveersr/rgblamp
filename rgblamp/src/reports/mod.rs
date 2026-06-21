//! References:
//!
//! HID Spec: https://www.usb.org/document-library/device-class-definition-hid-111
//! HUT:      https://usb.org/document-library/hid-usage-tables-14
//!
//! The HUT (HID Usage Tables) document has the information for the LampArray interface
//! under Section 26: Lighting and Illumination Page.

use std::fmt::Debug;

use bilge::prelude::*;
use enum_dispatch::enum_dispatch;

use crate::{
    error::LampResult,
    reports::{
        lamp_array_attrs::LampArrayAttrsReport, lamp_array_control::LampArrayControlReport,
        lamp_attrs_request::LampAttrsRequestReport, lamp_attrs_response::LampAttrsResponseReport,
        lamp_multi_update::LampMultiUpdateReport, lamp_range_update::LampRangeUpdateReport,
    },
    utils::info::ReportInfo,
};

pub mod lamp_array_attrs;
pub mod lamp_array_control;
pub mod lamp_attrs_request;
pub mod lamp_attrs_response;
pub mod lamp_multi_update;
pub mod lamp_range_update;
pub mod parser;

#[enum_dispatch]
pub trait Report {
    fn register(&mut self, usages: &[u16], size: u32) -> LampResult<()>;
    fn validate(&self) -> LampResult<()>;
}

#[enum_dispatch(Report)]
pub enum ReportKind {
    ArrayAttrs(LampArrayAttrsReport),
    AttrsRequest(LampAttrsRequestReport),
    AttrsResponse(LampAttrsResponseReport),
    MultiUpdate(LampMultiUpdateReport),
    RangeUpdate(LampRangeUpdateReport),
    ArrayControl(LampArrayControlReport),
}

#[derive(Debug)]
pub struct Reports {
    pub lamp_array_attrs: LampArrayAttrsReport,
    pub lamp_attrs_request: LampAttrsRequestReport,
    pub lamp_attrs_response: LampAttrsResponseReport,
    pub lamp_multi_update: LampMultiUpdateReport,
    pub lamp_range_update: LampRangeUpdateReport,
    pub lamp_array_control: LampArrayControlReport,
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
