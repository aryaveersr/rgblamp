pub const PAGE_LIGHTING: u16 = 0x59;

pub const LAMP_ARRAY: u16 = 0x1;
pub const LAMP_ARRAY_ATTRIBUTES_REPORT: u16 = 0x2;
pub const LAMP_COUNT: u16 = 0x3;

#[expect(unused)]
pub const LAMP_ARRAY_KIND: u16 = 0x7;

pub const MIN_UPDATE_INTERVAL_US: u16 = 0x8;
pub const LAMP_ATTRIBUTES_REQUEST_REPORT: u16 = 0x20;
pub const LAMP_ID: u16 = 0x21;
pub const LAMP_ATTRIBUTES_RESPONSE_REPORT: u16 = 0x22;
pub const UPDATE_LATENCY_US: u16 = 0x27;
pub const RED_LEVEL_COUNT: u16 = 0x28;
pub const GREEN_LEVEL_COUNT: u16 = 0x29;
pub const BLUE_LEVEL_COUNT: u16 = 0x2A;
pub const INTENSITY_LEVEL_COUNT: u16 = 0x2B;
pub const IS_PROGRAMMABLE: u16 = 0x2C;
pub const LAMP_MULTI_UPDATE_REPORT: u16 = 0x50;
pub const RED_UPDATE_CHANNEL: u16 = 0x51;
pub const GREEN_UPDATE_CHANNEL: u16 = 0x52;
pub const BLUE_UPDATE_CHANNEL: u16 = 0x53;
pub const INTENSITY_UPDATE_CHANNEL: u16 = 0x54;
pub const LAMP_UPDATE_FLAGS: u16 = 0x55;
pub const LAMP_RANGE_UPDATE_REPORT: u16 = 0x60;
pub const LAMP_ID_START: u16 = 0x61;
pub const LAMP_ID_END: u16 = 0x62;
pub const LAMP_ARRAY_CONTROL_REPORT: u16 = 0x70;
pub const AUTONOMOUS_MODE: u16 = 0x71;
