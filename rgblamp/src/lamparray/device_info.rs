use std::{collections::HashMap, str::FromStr};

use bilge::prelude::*;

use crate::error::{Error, LampResult};

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub dev_name: String,
    pub hid_id: HidId,
    pub hid_name: Option<String>,
    pub hid_phys: Option<String>,
    pub hid_uniq: Option<String>,
}

impl DeviceInfo {
    pub fn new(dev_props: &str, hid_props: &str) -> LampResult<Self> {
        let dev_props: HashMap<_, _> = dev_props
            .lines()
            .filter_map(|line| line.split_once('='))
            .filter(|(_, v)| !v.is_empty())
            .collect();

        let hid_props: HashMap<_, _> = hid_props
            .lines()
            .filter_map(|line| line.split_once('='))
            .filter(|(_, v)| !v.is_empty())
            .collect();

        let dev_name = *dev_props
            .get("DEVNAME")
            .ok_or(Error::device_parser("DEVNAME not found"))?;

        let hid_id = hid_props
            .get("HID_ID")
            .ok_or(Error::device_parser("HID_ID not found"))?;

        Ok(Self {
            dev_name: dev_name.to_owned(),
            hid_id: HidId::from_str(hid_id)?,
            hid_name: hid_props.get("HID_NAME").copied().map(str::to_string),
            hid_phys: hid_props.get("HID_PHYS").copied().map(str::to_string),
            hid_uniq: hid_props.get("HID_UNIQ").copied().map(str::to_string),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HidId {
    pub bus_type: BusType,
    pub vendor: u32,
    pub product: u32,
}

impl FromStr for HidId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, ":");

        let bus_type = BusType::from_str(
            parts
                .next()
                .ok_or_else(|| Error::device_parser("invalid hid id"))?,
        )?;

        let vendor = u32::from_str_radix(
            parts
                .next()
                .ok_or_else(|| Error::device_parser("invalid hid id"))?,
            16,
        )
        .map_err(|_| Error::device_parser("invalid hid id"))?;

        let product = u32::from_str_radix(
            parts
                .next()
                .ok_or_else(|| Error::device_parser("invalid hid id"))?,
            16,
        )
        .map_err(|_| Error::device_parser("invalid hid id"))?;

        Ok(Self {
            bus_type,
            vendor,
            product,
        })
    }
}

// Linux Kernel bus types.
// Source: https://github.com/torvalds/linux/blob/ef0c9f75a19532d7675384708fc8621e10850104/include/uapi/linux/input.h#L254-L278
#[bitsize(16)]
#[repr(u16)]
#[derive(FromBits, Debug, Clone, Copy)]
pub enum BusType {
    Pci = 0x01,
    Isapnp,
    Usb,
    Hil,
    Bluetooth,
    Virtual,
    Isa = 0x10,
    I8042,
    Xtkbd,
    Rs232,
    Gameport,
    Parport,
    Amiga,
    Adb,
    I2c,
    Host,
    Gsc,
    Atari,
    Spi,
    Rmi,
    Cec,
    IntelIshtp,
    AmdSfh,
    Sdw,

    #[fallback]
    Unknown(u16),
}

impl FromStr for BusType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(
            u16::from_str_radix(s, 16).map_err(|_| Error::device_parser("invalid hid id"))?,
        ))
    }
}
