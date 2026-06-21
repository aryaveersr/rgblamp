//! The Report Descriptor format is covered in Section 6 of HID Spec.

use bilge::prelude::*;

use crate::{
    reports::{
        Report, Reports, lamp_array_attrs::LampArrayAttrsReport,
        lamp_array_control::LampArrayControlReport, lamp_attrs_request::LampAttrsRequestReport,
        lamp_attrs_response::LampAttrsResponseReport, lamp_multi_update::LampMultiUpdateReport,
        lamp_range_update::LampRangeUpdateReport,
    },
    utils::usage,
};

#[derive(Default)]
pub struct ReportDescriptorParser<'a> {
    // Reference to the next byte to be parsed.
    bytes: &'a [u8],

    // States.
    globals: GlobalState,
    global_stack: Vec<GlobalState>,
    usages: Vec<u16>,
    usage_range_min: Option<u16>,
    collection_depth: usize,

    // LampArray.
    //
    // Root depth decides whether we're in a collection of a LampArray or not.
    root_depth: Option<usize>,
    report_kind: Option<ReportKind>,
    // Reports.
    lamp_array_attrs_report: Option<LampArrayAttrsReport>,
    lamp_attrs_request_report: Option<LampAttrsRequestReport>,
    lamp_attrs_response_report: Option<LampAttrsResponseReport>,
    lamp_multi_update_report: Option<LampMultiUpdateReport>,
    lamp_range_update_report: Option<LampRangeUpdateReport>,
    lamp_array_control_report: Option<LampArrayControlReport>,
}

impl Iterator for ReportDescriptorParser<'_> {
    type Item = Reports;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((tag, data)) = self.next_item() {
            match tag.kind() {
                ItemKind::Global => self.handle_global_item(tag.tag(), data),
                ItemKind::Local => self.handle_local_item(tag.tag(), data),
                ItemKind::Main => {
                    if let Some(report) = self.handle_main_item(tag.tag()) {
                        return Some(report);
                    }
                }
                ItemKind::Reserved => panic!(),
            }
        }

        None
    }
}

impl<'a> ReportDescriptorParser<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            ..Default::default()
        }
    }

    fn handle_global_item(&mut self, kind: u4, data: u32) {
        let kind = GlobalItemKind::try_from(kind).unwrap();
        match kind {
            GlobalItemKind::UsagePage => self.globals.usage_page = Some(data as u16),
            GlobalItemKind::LogicalMin => self.globals.logical_min = Some(data as i32),
            GlobalItemKind::LogicalMax => self.globals.logical_max = Some(data as i32),
            GlobalItemKind::ReportID => self.globals.report_id = data as u8,
            GlobalItemKind::ReportSize => self.globals.report_size = Some(data),
            GlobalItemKind::ReportCount => self.globals.report_count = Some(data),
            GlobalItemKind::Push => self.global_stack.push(self.globals.clone()),
            GlobalItemKind::Pop => self.globals = self.global_stack.pop().unwrap(),
            _ => (),
        }
    }

    fn handle_local_item(&mut self, kind: u4, data: u32) {
        let kind = LocalItemKind::try_from(kind).unwrap();
        match kind {
            LocalItemKind::Usage => {
                assert!(data <= u16::MAX as u32);
                self.usages.push(data as u16);
            }
            LocalItemKind::UsageMin => {
                assert!(data <= u16::MAX as u32);
                self.usage_range_min = Some(data as u16);
            }
            LocalItemKind::UsageMax => {
                let min = self.usage_range_min.take().unwrap();
                let max = data as u16;
                self.usages.extend(min..=max);
            }
            _ => (),
        }
    }

    fn handle_main_item(&mut self, kind: u4) -> Option<Reports> {
        let kind = MainItemKind::try_from(kind).unwrap();
        match kind {
            MainItemKind::Collection => self.start_collection(),
            MainItemKind::EndCollection => return self.end_collection(),
            MainItemKind::Input => self.handle_data_item(DataKind::Input),
            MainItemKind::Output => self.handle_data_item(DataKind::Output),
            MainItemKind::Feature => self.handle_data_item(DataKind::Feature),
        }

        None
    }

    fn handle_data_item(&mut self, kind: DataKind) {
        if let Some(report_kind) = self.report_kind {
            let size = self.globals.report_size.unwrap();
            let count = self.globals.report_count.unwrap() as usize;

            assert_ne!(kind, DataKind::Input);
            assert_ne!(kind, DataKind::Output, "TODO");
            assert!(!self.usages.is_empty());
            assert!(self.usages.len() <= count);

            let mut usages = std::mem::take(&mut self.usages);

            // Remarks from Section 6.2.2.8 (Local Items) from HID Spec.
            // If there are more controls than usages, the last usage applies
            // to all remaining controls.
            if usages.len() < count {
                let last_usage = *usages.last().unwrap();
                usages.resize(count, last_usage);
            }

            self.get_report(report_kind).register(&usages, size);
            self.usages = usages;
        }

        self.usages.clear();
    }

    fn start_collection(&mut self) {
        self.collection_depth += 1;

        if self.globals.usage_page != Some(usage::PAGE_LIGHTING) {
            return;
        }

        let id = self.globals.report_id;
        let usage = self.usages.pop().unwrap();

        match usage {
            usage::LAMP_ARRAY => self.root_depth = Some(self.collection_depth),
            usage::LAMP_ARRAY_ATTRIBUTES_REPORT => {
                self.lamp_array_attrs_report = Some(LampArrayAttrsReport::new(id));
                self.report_kind = Some(ReportKind::ArrayAttrs);
            }
            usage::LAMP_ATTRIBUTES_REQUEST_REPORT => {
                self.lamp_attrs_request_report = Some(LampAttrsRequestReport::new(id));
                self.report_kind = Some(ReportKind::AttrsRequest);
            }
            usage::LAMP_ATTRIBUTES_RESPONSE_REPORT => {
                self.lamp_attrs_response_report = Some(LampAttrsResponseReport::new(id));
                self.report_kind = Some(ReportKind::AttrsResponse);
            }
            usage::LAMP_MULTI_UPDATE_REPORT => {
                self.lamp_multi_update_report = Some(LampMultiUpdateReport::new(id));
                self.report_kind = Some(ReportKind::MultiUpdate);
            }
            usage::LAMP_RANGE_UPDATE_REPORT => {
                self.lamp_range_update_report = Some(LampRangeUpdateReport::new(id));
                self.report_kind = Some(ReportKind::RangeUpdate);
            }
            usage::LAMP_ARRAY_CONTROL_REPORT => {
                self.lamp_array_control_report = Some(LampArrayControlReport::new(id));
                self.report_kind = Some(ReportKind::ArrayControlReport)
            }
            _ => self.report_kind = None,
        }
    }

    fn end_collection(&mut self) -> Option<Reports> {
        assert!(self.collection_depth > 0);

        if let Some(kind) = self.report_kind {
            self.get_report(kind).info().validate();
        }

        self.collection_depth -= 1;
        self.usages.clear();

        match self.root_depth {
            Some(depth) if depth == self.collection_depth + 1 => {
                self.root_depth = None;
                Some(Reports {
                    lamp_array_attrs: self.lamp_array_attrs_report.take().unwrap(),
                    lamp_attrs_request: self.lamp_attrs_request_report.take().unwrap(),
                    lamp_attrs_response: self.lamp_attrs_response_report.take().unwrap(),
                    lamp_multi_update: self.lamp_multi_update_report.take().unwrap(),
                    lamp_range_update: self.lamp_range_update_report.take().unwrap(),
                    lamp_array_control: self.lamp_array_control_report.take().unwrap(),
                })
            }
            _ => None,
        }
    }

    fn next_item(&mut self) -> Option<(ItemTag, u32)> {
        if self.bytes.is_empty() {
            return None;
        }

        let tag = ItemTag::from(self.bytes[0]);
        let len = match tag.size() {
            ItemSize::None => 0,
            ItemSize::One => 1,
            ItemSize::Two => 2,
            ItemSize::Four => 4,
        };

        assert!(self.bytes.len() > len);

        let slice = &self.bytes[1..(1 + len)];
        self.bytes = &self.bytes[(1 + len)..];

        let mut buffer = [0u8; 4];
        buffer[..len].copy_from_slice(slice);

        Some((tag, u32::from_le_bytes(buffer)))
    }

    fn get_report(&mut self, kind: ReportKind) -> &mut dyn Report {
        match kind {
            ReportKind::ArrayAttrs => self.lamp_array_attrs_report.as_mut().unwrap(),
            ReportKind::MultiUpdate => self.lamp_multi_update_report.as_mut().unwrap(),
            ReportKind::RangeUpdate => self.lamp_range_update_report.as_mut().unwrap(),
            ReportKind::ArrayControlReport => self.lamp_array_control_report.as_mut().unwrap(),
            ReportKind::AttrsRequest => self.lamp_attrs_request_report.as_mut().unwrap(),
            ReportKind::AttrsResponse => self.lamp_attrs_response_report.as_mut().unwrap(),
        }
    }
}

/// Global state that can be modified by "Global Items".
/// (see Section 6.2.2.7 HID Spec)
#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub usage_page: Option<u16>,
    pub logical_min: Option<i32>,
    pub logical_max: Option<i32>,

    // If no Report IDs are used, the default is 0.
    pub report_id: u8,
    pub report_size: Option<u32>,
    pub report_count: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
enum DataKind {
    Input,
    Output,
    Feature,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ReportKind {
    ArrayAttrs,
    AttrsRequest,
    AttrsResponse,
    MultiUpdate,
    RangeUpdate,
    ArrayControlReport,
}

// Section 6.2.2.2 of HID Spec.
//
// Tags for Short Items.
// These definitions allow direct parsing from raw bytes.
#[bitsize(8)]
#[derive(FromBits, DebugBits)]
struct ItemTag {
    pub size: ItemSize,
    pub kind: ItemKind,
    pub tag: u4,
}

#[bitsize(2)]
#[derive(FromBits, Debug)]
enum ItemSize {
    None,
    One,
    Two,
    Four,
}

#[bitsize(2)]
#[derive(FromBits, Debug)]
enum ItemKind {
    Main,
    Global,
    Local,
    Reserved,
}

#[bitsize(4)]
#[derive(TryFromBits)]
enum MainItemKind {
    Input = 0b1000,
    Output = 0b1001,
    Feature = 0b1011,
    Collection = 0b1010,
    EndCollection = 0b1100,
}

#[bitsize(4)]
#[derive(TryFromBits)]
enum GlobalItemKind {
    UsagePage,
    LogicalMin,
    LogicalMax,
    PhysicalMin,
    PhysicalMax,
    UnitExponent,
    Unit,
    ReportSize,
    ReportID,
    ReportCount,
    Push,
    Pop,
}

#[bitsize(4)]
#[derive(TryFromBits)]
enum LocalItemKind {
    Usage,
    UsageMin,
    UsageMax,
    DesignatorIndex,
    DesignatorMin,
    DesignatorMax,
    StringIndex = 0b111,
    StringMin,
    StringMax,
    Delimiter,
}
