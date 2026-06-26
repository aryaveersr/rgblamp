use std::fs::File;

use crate::{
    error::LampResult,
    reports::{
        Report, ReportInfo,
        utils::{
            field::{Buffer, ReportField},
            io::set_feature,
            usage,
        },
    },
};

#[derive(Debug, Default)]
pub struct LampAttrsRequestReport {
    info: ReportInfo,

    lamp_id: ReportField,
}

impl Report for LampAttrsRequestReport {
    fn register(&mut self, usages: &[u16], size: u32) -> LampResult<()> {
        for usage in usages {
            let args = self.info.increment(size);
            if *usage == usage::LAMP_ID {
                self.lamp_id.set(args)?;
            }
        }

        Ok(())
    }

    fn validate(&self) -> LampResult<()> {
        self.info.validate()?;

        self.lamp_id.validate("LAMP_ID")?;
        Ok(())
    }
}

impl LampAttrsRequestReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, lamp_id: u32) -> LampResult<()> {
        let mut buffer = Buffer::new(&self.info);

        self.lamp_id.write(&mut buffer, lamp_id);

        set_feature(file, &mut buffer)
    }
}
