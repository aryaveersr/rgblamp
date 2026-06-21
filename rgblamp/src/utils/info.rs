use crate::utils::field::ReportField;

#[derive(Debug, Default)]
pub struct ReportInfo {
    pub id: u8,
    pub size: u32,
}

impl ReportInfo {
    pub fn new(id: u8) -> Self {
        Self { id, size: 0 }
    }

    pub fn validate(&self) {
        assert_eq!(self.size % 8, 0)
    }

    pub fn bytes_len(&self) -> usize {
        self.size as usize / 8
    }

    pub fn create_field(&mut self, size: u32) -> ReportField {
        let field = ReportField::new(self.size, size);
        self.size += size;
        field
    }
}
