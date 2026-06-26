use std::{collections::HashMap, str::FromStr};

use anyhow::Context;
use bilge::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HidInfo {
    pub id: HidId,
    pub name: Option<String>,
    pub phys: Option<String>,
    pub uniq: Option<String>,
}

impl FromStr for HidInfo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props: HashMap<_, _> = s
            .lines()
            .filter_map(|line| line.split_once('='))
            .filter(|(_, v)| !v.is_empty())
            .collect();

        let id = props.get("HID_ID").context("HID_ID not found")?;

        Ok(Self {
            id: HidId::from_str(id)?,
            name: props.get("HID_NAME").copied().map(str::to_string),
            phys: props.get("HID_PHYS").copied().map(str::to_string),
            uniq: props.get("HID_UNIQ").copied().map(str::to_string),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HidId {
    pub bus_type: BusType,
    pub vendor: u32,
    pub product: u32,
}

impl FromStr for HidId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, ":");

        let bus_type = BusType::from_str(parts.next().context("invalid hid id")?)?;
        let vendor = u32::from_str_radix(parts.next().context("invalid hid id")?, 16)?;
        let product = u32::from_str_radix(parts.next().context("invalid hid id")?, 16)?;

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
#[derive(FromBits, Debug, Clone, Copy, Serialize)]
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(u16::from_str_radix(s, 16)?))
    }
}
