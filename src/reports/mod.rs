//! References:
//!
//! HID Spec: https://www.usb.org/document-library/device-class-definition-hid-111
//! HUT:      https://usb.org/document-library/hid-usage-tables-14
//!
//! The HUT (HID Usage Tables) document has the information for the LampArray interface
//! under Section 26: Lighting and Illumination Page.

use std::{fs::File, os::fd::AsRawFd};

use nix::{ioctl_read, ioctl_readwrite_buf};

use crate::reports::parser::ReportDescriptorParser;

mod parser;

ioctl_readwrite_buf!(hid_get_feature, b'H', 0x07, u8);

pub struct Reports {
    pub lamp_array_attributes: LampArrayAttributesReport,
}

impl Reports {
    pub fn from_descriptor(bytes: &[u8]) -> Option<Self> {
        ReportDescriptorParser::new(bytes).parse()
    }
}

#[derive(Debug, Default)]
pub struct LampArrayAttributesReport {
    pub(self) info: ReportInfo,
    pub(self) lamp_count: ReportField,
    pub(self) min_update_interval_us: ReportField,
}

impl LampArrayAttributesReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File) -> LampArrayAttributes {
        assert_eq!(self.info.size % 8, 0);
        let mut buffer = [0u8; 23];
        buffer[0] = self.info.id;
        unsafe {
            hid_get_feature(file.as_raw_fd(), &mut buffer).unwrap();
        }
        let bytes = &buffer[1..];
        let lamp_count = self.lamp_count.extract(bytes);
        let min_update_interval_us = self.min_update_interval_us.extract(bytes);

        LampArrayAttributes {
            lamp_count,
            min_update_interval_us,
        }
    }
}

#[derive(Debug)]
pub struct LampArrayAttributes {
    pub lamp_count: u32,
    pub min_update_interval_us: u32,
}

// ReportKind::LampAttributesRequest => todo!(),
// ReportKind::LampAttributesResponse => todo!(),
// ReportKind::LampMultiUpdate => todo!(),
// ReportKind::LampRangeUpdate => todo!(),
// ReportKind::LampArrayControlReport => todo!(),

#[derive(Debug, Default)]
pub struct LampAttributesRequestReport {
    pub(self) info: ReportInfo,
    pub(self) lamp_id: ReportField,
}

impl LampAttributesRequestReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, lamp_id: u8) {
        dbg!(lamp_id, self);
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct LampAttributesResponseReport {
    pub(self) info: ReportInfo,
    pub(self) lamp_id: ReportField,
    pub(self) update_latency_us: ReportField,
    pub(self) red_level_count: ReportField,
    pub(self) green_level_count: ReportField,
    pub(self) blue_level_count: ReportField,
    pub(self) intensity_level_count: ReportField,
    pub(self) is_programmable: ReportField,
}

impl LampAttributesResponseReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File) -> LampAttributes {
        dbg!(self);
        todo!()
    }
}

pub struct LampAttributes {
    pub lamp_id: u8,
    pub update_latency_us: u32,
    pub red_level_count: u32,
    pub green_level_count: u32,
    pub blue_level_count: u32,
    pub intensity_level_count: u32,
    pub is_programmable: bool,
}

#[derive(Debug, Default)]
pub struct LampMultiUpdateReport {
    pub(self) info: ReportInfo,
    pub(self) slots: u32,
    pub(self) lamp_count: ReportField,
    pub(self) lamp_update_flags: ReportField,
    pub(self) lamp_id_first: ReportField,
    pub(self) red_update_channel_first: ReportField,
    pub(self) green_update_channel_first: ReportField,
    pub(self) blue_update_channel_first: ReportField,
    pub(self) intensity_update_channel_first: ReportField,
}

impl LampMultiUpdateReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, params: LampMultiUpdateParams) {
        dbg!(self, params);
        todo!()
    }
}

#[derive(Debug)]
pub struct LampMultiUpdateParams<'a> {
    pub lamp_update_flags: u16,
    pub items: &'a [LampMultiUpdateItem],
}

#[derive(Debug)]
pub struct LampMultiUpdateItem {
    pub lamp_id: u8,
    pub red_update_channel: u32,
    pub green_update_channel: u32,
    pub blue_update_channel: u32,
    pub intensity_update_channel: u32,
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

    pub fn extract(&self, bytes: &[u8]) -> u32 {
        assert!(bytes.len() >= self.size + self.offset);
        let mut buffer = [0u8; 4];
        buffer[..self.size].copy_from_slice(&bytes[self.offset..(self.offset + self.size)]);
        u32::from_le_bytes(buffer)
    }
}

#[derive(Debug, Default)]
pub(self) struct ReportInfo {
    pub(self) id: u8,
    pub(self) size: u32,
}

impl ReportInfo {
    pub(self) fn new(id: u8) -> Self {
        Self { id, size: 0 }
    }
}
