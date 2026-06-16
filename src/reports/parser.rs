//! The Report Descriptor format is covered in Section 6 of HID Spec.

use bilge::prelude::*;

use crate::reports::{
    LampArrayAttrsReport, LampAttrsRequestReport, LampAttrsResponseReport, LampMultiUpdateReport,
    Report, Reports, consts, lamp_array_control::LampArrayControlReport,
    lamp_range_update::LampRangeUpdateReport,
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

impl<'a> ReportDescriptorParser<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            ..Default::default()
        }
    }

    pub fn parse(mut self) -> Reports {
        while let Some((tag, data)) = self.next() {
            match tag.kind() {
                ItemKind::Global => self.handle_global_item(tag.tag(), data),
                ItemKind::Local => self.handle_local_item(tag.tag(), data),
                ItemKind::Main => self.handle_main_item(tag.tag()),
                ItemKind::Reserved => panic!(),
            }
        }

        Reports {
            lamp_array_attrs: self.lamp_array_attrs_report.unwrap(),
            lamp_attrs_request: self.lamp_attrs_request_report.unwrap(),
            lamp_attrs_response: self.lamp_attrs_response_report.unwrap(),
            lamp_multi_update: self.lamp_multi_update_report.unwrap(),
            lamp_range_update: self.lamp_range_update_report.unwrap(),
            lamp_array_control: self.lamp_array_control_report.unwrap(),
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

    fn handle_main_item(&mut self, kind: u4) {
        let kind = MainItemKind::try_from(kind).unwrap();
        match kind {
            MainItemKind::Collection => self.start_collection(),
            MainItemKind::EndCollection => self.end_collection(),
            MainItemKind::Input => self.handle_data_item(DataKind::Input),
            MainItemKind::Output => self.handle_data_item(DataKind::Output),
            MainItemKind::Feature => self.handle_data_item(DataKind::Feature),
        }
    }

    fn handle_data_item(&mut self, kind: DataKind) {
        if let Some(report_kind) = self.report_kind {
            let size = self.globals.report_size.unwrap();
            let count = self.globals.report_count.unwrap();

            assert_ne!(kind, DataKind::Input);
            assert_ne!(kind, DataKind::Output, "TODO");
            assert_eq!(self.usages.len() as u32, count);

            let usages = std::mem::take(&mut self.usages);
            self.get_report(report_kind).register(&usages, size);
            self.usages = usages;
        }

        self.usages.clear();
    }

    fn start_collection(&mut self) {
        self.collection_depth += 1;

        let id = self.globals.report_id;
        let usage = self.usages.pop().unwrap();

        match usage {
            consts::USAGE_LAMP_ARRAY => self.root_depth = Some(self.collection_depth),
            consts::USAGE_LAMP_ARRAY_ATTRIBUTES_REPORT => {
                self.lamp_array_attrs_report = Some(LampArrayAttrsReport::new(id));
                self.report_kind = Some(ReportKind::LampArrayAttrs);
            }
            consts::USAGE_LAMP_ATTRIBUTES_REQUEST_REPORT => {
                self.lamp_attrs_request_report = Some(LampAttrsRequestReport::new(id));
                self.report_kind = Some(ReportKind::LampAttrsRequest);
            }
            consts::USAGE_LAMP_ATTRIBUTES_RESPONSE_REPORT => {
                self.lamp_attrs_response_report = Some(LampAttrsResponseReport::new(id));
                self.report_kind = Some(ReportKind::LampAttrsResponse);
            }
            consts::USAGE_LAMP_MULTI_UPDATE_REPORT => {
                self.lamp_multi_update_report = Some(LampMultiUpdateReport::new(id));
                self.report_kind = Some(ReportKind::LampMultiUpdate);
            }
            consts::USAGE_LAMP_RANGE_UPDATE_REPORT => {
                self.lamp_range_update_report = Some(LampRangeUpdateReport::new(id));
                self.report_kind = Some(ReportKind::LampRangeUpdate);
            }
            consts::USAGE_LAMP_ARRAY_CONTROL_REPORT => {
                self.lamp_array_control_report = Some(LampArrayControlReport::new(id));
                self.report_kind = Some(ReportKind::LampArrayControlReport)
            }
            _ => self.report_kind = None,
        }
    }

    fn end_collection(&mut self) {
        assert!(self.collection_depth > 0);

        if let Some(kind) = self.report_kind {
            self.get_report(kind).get_info().validate();
        }

        if Some(self.collection_depth) == self.root_depth {
            self.root_depth = None;
        }

        self.collection_depth -= 1;
        self.usages.clear();
    }

    fn next(&mut self) -> Option<(ItemTag, u32)> {
        if self.bytes.is_empty() {
            return None;
        }

        let item_tag = ItemTag::from(self.bytes[0]);
        let data_len = match item_tag.size() {
            ItemSize::None => 0,
            ItemSize::One => 1,
            ItemSize::Two => 2,
            ItemSize::Four => 4,
        };

        if self.bytes.len() < data_len + 1 {
            return None;
        }

        if data_len == 0 {
            self.bytes = &self.bytes[1..];
            return Some((item_tag, 0));
        }

        let data = {
            let slice = &self.bytes[1..(1 + data_len)];
            self.bytes = &self.bytes[(1 + data_len)..];

            let mut buffer = [0u8; 4];
            buffer[..data_len].copy_from_slice(slice);

            u32::from_le_bytes(buffer)
        };

        Some((item_tag, data))
    }

    fn get_report(&mut self, kind: ReportKind) -> &mut dyn Report {
        match kind {
            ReportKind::LampArrayAttrs => self.lamp_array_attrs_report.as_mut().unwrap(),
            ReportKind::LampMultiUpdate => self.lamp_multi_update_report.as_mut().unwrap(),
            ReportKind::LampRangeUpdate => self.lamp_range_update_report.as_mut().unwrap(),
            ReportKind::LampArrayControlReport => self.lamp_array_control_report.as_mut().unwrap(),
            ReportKind::LampAttrsRequest => self.lamp_attrs_request_report.as_mut().unwrap(),
            ReportKind::LampAttrsResponse => self.lamp_attrs_response_report.as_mut().unwrap(),
        }
    }
}

/// Global state that can be modified by "Global Items".
/// (see Section 6.2.2.7 HID Spec)
#[derive(Debug, Clone)]
pub struct GlobalState {
    pub usage_page: Option<u16>,
    pub logical_min: Option<i32>,
    pub logical_max: Option<i32>,

    // If no Report IDs are used, the default is 0.
    pub report_id: u8,
    pub report_size: Option<u32>,
    pub report_count: Option<u32>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            usage_page: None,
            logical_min: None,
            logical_max: None,
            report_id: 0,
            report_size: None,
            report_count: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DataKind {
    Input,
    Output,
    Feature,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ReportKind {
    LampArrayAttrs,
    LampAttrsRequest,
    LampAttrsResponse,
    LampMultiUpdate,
    LampRangeUpdate,
    LampArrayControlReport,
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
