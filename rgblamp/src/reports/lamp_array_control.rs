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
pub struct LampArrayControlReport {
    info: ReportInfo,
    auto_mode: ReportField<bool>,
}

impl Report for LampArrayControlReport {
    fn register(&mut self, usages: &[u16], size: u32) -> LampResult<()> {
        for usage in usages {
            let args = self.info.increment(size);
            if *usage == usage::AUTONOMOUS_MODE {
                self.auto_mode.set(args)?;
            }
        }

        Ok(())
    }

    fn validate(&self) -> LampResult<()> {
        self.info.validate()?;

        self.auto_mode.validate("AUTONOMOUS_MODE")?;
        Ok(())
    }
}

impl LampArrayControlReport {
    pub fn new(id: u8) -> Self {
        Self {
            info: ReportInfo::new(id),
            ..Default::default()
        }
    }

    pub fn send(&self, file: &mut File, auto_mode: bool) -> LampResult<()> {
        let mut buffer = Buffer::new(&self.info);

        self.auto_mode.write(&mut buffer, auto_mode);

        set_feature(file, &mut buffer)
    }
}
