use std::{
    fs,
    ops::{Deref, DerefMut},
};

use log::trace;

use crate::{
    LampArray, LampResult, lamparray::device_info::DeviceInfo,
    reports::parser::ReportDescriptorParser,
};

#[derive(Debug)]
pub struct LampArrays {
    lamparrays: Vec<LampArray>,
}

impl LampArrays {
    pub fn new() -> LampResult<Self> {
        trace!("Enumerating LampArray devices");

        let mut lamparrays = Vec::new();
        let entries = fs::read_dir("/sys/class/hidraw")?.collect::<Result<Vec<_>, _>>()?;

        for entry in entries {
            let info = {
                let dev_path = entry.path().join("uevent");
                let hid_path = entry.path().join("device/uevent");
                DeviceInfo::new(
                    &fs::read_to_string(dev_path)?,
                    &fs::read_to_string(hid_path)?,
                )?
            };

            let path = entry.path().join("device/report_descriptor");
            let contents = fs::read(path)?;
            let mut parser = ReportDescriptorParser::new(&contents);

            while let Some(reports) = parser.next()? {
                lamparrays.push(LampArray::new(info.clone(), reports)?);
            }
        }

        Ok(Self { lamparrays })
    }
}

impl Deref for LampArrays {
    type Target = Vec<LampArray>;

    fn deref(&self) -> &Self::Target {
        &self.lamparrays
    }
}

impl DerefMut for LampArrays {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lamparrays
    }
}
