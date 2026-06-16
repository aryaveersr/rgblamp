//! References:
//!
//! HID Spec: https://www.usb.org/document-library/device-class-definition-hid-111
//! HUT:      https://usb.org/document-library/hid-usage-tables-14
//!
//! The HUT (HID Usage Tables) document has the information for the LampArray interface
//! under Section 26: Lighting and Illumination Page.

use std::{fmt::Debug, marker::PhantomData};

use crate::reports::{
    lamp_array_attributes::LampArrayAttributesReport, lamp_array_control::LampArrayControlReport,
    lamp_attributes_request::LampAttributesRequestReport,
    lamp_attributes_response::LampAttributesResponseReport,
    lamp_multi_update::LampMultiUpdateReport, lamp_range_update::LampRangeUpdateReport,
    parser::ReportDescriptorParser,
};

pub mod lamp_array_attributes;
pub mod lamp_array_control;
pub mod lamp_attributes_request;
pub mod lamp_attributes_response;
pub mod lamp_multi_update;
pub mod lamp_range_update;

mod io;
mod parser;

pub struct Reports {
    pub lamp_array_attributes: LampArrayAttributesReport,
    pub lamp_attributes_request: LampAttributesRequestReport,
    pub lamp_attributes_response: LampAttributesResponseReport,
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
pub(self) struct ReportField<T = u32>
where
    T: Into<u32>,
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: Debug,
{
    pub(self) offset: usize,
    pub(self) size: usize,
    _phantom: PhantomData<T>,
}

impl<T> ReportField<T>
where
    T: Into<u32>,
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: Debug,
{
    pub fn new(offset_bits: u32, size_bits: u32) -> Self {
        assert_eq!(offset_bits % 8, 0);
        assert_eq!(size_bits % 8, 0);

        let offset = offset_bits as usize / 8;
        let size = size_bits as usize / 8;

        assert!(size <= std::mem::size_of::<T>());

        Self {
            offset,
            size,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self, bytes: &[u8]) -> T {
        assert!(bytes.len() >= self.size + self.offset);
        let mut buffer = [0u8; 4];
        buffer[..self.size].copy_from_slice(&bytes[self.offset..(self.offset + self.size)]);
        u32::from_le_bytes(buffer).try_into().unwrap()
    }

    pub fn set(&self, bytes: &mut [u8], value: T) {
        assert!(bytes.len() >= self.size + self.offset);
        let value = value.into().to_le_bytes();
        bytes[..self.size].copy_from_slice(&value[..self.size]);
    }
}

#[derive(Debug, Default)]
pub(self) struct ReportInfo {
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
}

pub(self) trait Report {
    fn get_info(&self) -> &ReportInfo;
    fn get_info_mut(&mut self) -> &mut ReportInfo;
}
