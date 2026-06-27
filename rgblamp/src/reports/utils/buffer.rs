use crate::reports::utils::info::ReportInfo;

#[derive(Debug)]
pub struct Buffer {
    bytes: Vec<u8>,
}

impl Buffer {
    pub fn new(info: &ReportInfo) -> Self {
        let mut bytes = vec![0u8; 1 + info.bytes_len()];
        bytes[0] = info.id;
        Self { bytes }
    }

    pub(super) fn body(&self) -> &[u8] {
        &self.bytes[1..]
    }

    pub(super) fn body_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[1..]
    }

    pub(super) fn as_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}
