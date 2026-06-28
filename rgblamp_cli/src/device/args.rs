use std::{fs, str::FromStr};

use anyhow::Context;
use rgblamp::{LampArray, ReportDescriptorParser};

use crate::device::hid_info::HidInfo;

#[derive(clap::Args, Debug)]
#[command(next_help_heading = "Device selectors")]
pub struct DeviceArgs {
    /// The path name for the device under /dev
    #[arg(long, global = true)]
    dev_name: Option<String>,

    /// Device vendor id
    #[arg(long, global = true)]
    device_vendor: Option<u32>,

    /// Device product id
    #[arg(long, global = true)]
    device_product: Option<u32>,

    /// Device physical address
    #[arg(long, global = true)]
    device_phys: Option<String>,

    /// Device unique address
    #[arg(long, global = true)]
    device_uniq: Option<String>,

    /// Device name
    #[arg(long, global = true)]
    device_name: Option<String>,
}

impl DeviceArgs {
    pub fn enumerate(&self) -> anyhow::Result<Vec<(HidInfo, LampArray)>> {
        let mut devices = Vec::new();

        for entry in fs::read_dir("/sys/class/hidraw")? {
            let entry = entry?;

            let file_name = entry.file_name();
            let dev_name = file_name
                .to_str()
                .context("invalid device name")?
                .to_owned();

            if let Some(dev_name_arg) = &self.dev_name
                && *dev_name_arg != dev_name
            {
                continue;
            }

            let info = HidInfo::from_str(&fs::read_to_string(entry.path().join("device/uevent"))?)?;
            if !self.satisfies(&info) {
                continue;
            }

            let contents = fs::read(entry.path().join("device/report_descriptor"))?;
            let mut parser = ReportDescriptorParser::new(&contents);

            while let Some(lamp_array) = parser.next(&dev_name)? {
                devices.push((info.clone(), lamp_array));
            }
        }

        Ok(devices)
    }

    fn satisfies(&self, info: &HidInfo) -> bool {
        if let Some(device_vendor) = &self.device_vendor
            && *device_vendor != info.id.vendor
        {
            return false;
        }

        if let Some(device_product) = &self.device_product
            && *device_product != info.id.product
        {
            return false;
        }

        if let Some(device_phys) = &self.device_phys
            && Some(device_phys) != info.phys.as_ref()
        {
            return false;
        }

        if let Some(device_uniq) = &self.device_uniq
            && Some(device_uniq) != info.uniq.as_ref()
        {
            return false;
        }

        if let Some(device_name) = &self.device_name
            && Some(device_name) != info.name.as_ref()
        {
            return false;
        }

        true
    }
}
