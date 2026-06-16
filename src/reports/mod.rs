//! References:
//!
//! HID Spec: https://www.usb.org/document-library/device-class-definition-hid-111
//! HUT:      https://usb.org/document-library/hid-usage-tables-14
//!
//! The HUT (HID Usage Tables) document has the information for the LampArray interface
//! under Section 26: Lighting and Illumination Page.

use std::fs::File;

use crate::reports::{
    lamp_array_attributes::LampArrayAttributesReport,
    lamp_attributes_request::LampAttributesRequestReport,
    lamp_attributes_response::LampAttributesResponseReport,
    lamp_multi_update::LampMultiUpdateReport, parser::ReportDescriptorParser,
};

pub mod lamp_array_attributes;
pub mod lamp_attributes_request;
pub mod lamp_attributes_response;
pub mod lamp_multi_update;

mod io;
mod parser;

pub struct Reports {
    pub lamp_array_attributes: LampArrayAttributesReport,
    pub lamp_attributes_request: LampAttributesRequestReport,
    pub lamp_attributes_response: LampAttributesResponseReport,
    pub lamp_multi_update: LampMultiUpdateReport,
}

impl Reports {
    pub fn from_descriptor(bytes: &[u8]) -> Option<Self> {
        ReportDescriptorParser::new(bytes).parse()
    }
}

#[derive(Debug, Default)]
pub(self) struct ReportField {
    pub(self) offset: usize,
    pub(self) size: usize,
}

impl ReportField {
    pub fn new(offset_bits: u32, size_bits: u32) -> Self {
        assert_eq!(offset_bits % 8, 0);
        assert_eq!(size_bits % 8, 0);
        let offset = offset_bits as usize / 8;
        let size = size_bits as usize / 8;
        Self { offset, size }
    }

    pub fn get(&self, bytes: &[u8]) -> u32 {
        assert!(bytes.len() >= self.size + self.offset);
        let mut buffer = [0u8; 4];
        buffer[..self.size].copy_from_slice(&bytes[self.offset..(self.offset + self.size)]);
        u32::from_le_bytes(buffer)
    }

    pub fn set(&self, bytes: &mut [u8], value: u32) {
        assert!(bytes.len() >= self.size + self.offset);
        let value = value.to_le_bytes();
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
