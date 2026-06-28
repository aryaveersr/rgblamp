use crate::error::Error;

#[derive(Debug, Default)]
pub struct ReportInfo {
    pub id: u8,
    pub size: u32,
}

impl ReportInfo {
    pub fn new(id: u8) -> Self {
        Self { id, size: 0 }
    }

    pub fn bytes_len(&self) -> usize {
        self.size as usize / 8
    }

    pub fn increment(&mut self, size: u32) -> (u32, u32) {
        let args = (self.size, size);
        self.size += size;
        args
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.size.is_multiple_of(8) {
            Ok(())
        } else {
            Err(Error::parser("report size is not byte-aligned"))
        }
    }
}
