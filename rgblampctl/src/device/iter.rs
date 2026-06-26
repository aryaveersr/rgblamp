use std::{
    fs::{self, ReadDir},
    str::FromStr,
};

use anyhow::Context;
use rgblamp::{LampArray, ReportDescriptorParser};

use crate::device::{args::DeviceArgs, hid_info::HidInfo};

#[derive(Debug)]
pub struct DeviceIter<'a> {
    read_dir: ReadDir,
    args: &'a DeviceArgs,
}

impl<'a> DeviceIter<'a> {
    pub fn new(args: &'a DeviceArgs) -> anyhow::Result<Self> {
        Ok(Self {
            read_dir: fs::read_dir("/sys/class/hidraw")?,
            args,
        })
    }

    fn next_device(&mut self) -> anyhow::Result<Option<(HidInfo, LampArray)>> {
        for entry in self.read_dir.by_ref() {
            let entry = entry?;

            let file_name = entry.file_name();
            let dev_name = file_name
                .to_str()
                .context("invalid device name")?
                .to_owned();

            if let Some(dev_name_arg) = self.args.dev_name()
                && *dev_name_arg != dev_name
            {
                continue;
            }

            let info = HidInfo::from_str(&fs::read_to_string(entry.path().join("device/uevent"))?)?;
            if !self.args.satisfies(&info) {
                continue;
            }

            let contents = fs::read(entry.path().join("device/report_descriptor"))?;
            let mut parser = ReportDescriptorParser::new(&contents);

            if let Some(lamp_array) = parser.next(dev_name)? {
                return Ok(Some((info, lamp_array)));
            }
        }

        Ok(None)
    }
}

impl Iterator for DeviceIter<'_> {
    type Item = anyhow::Result<(HidInfo, LampArray)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_device().transpose()
    }
}
