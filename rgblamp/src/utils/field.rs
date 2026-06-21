use std::{fmt::Debug, marker::PhantomData, ops::AddAssign};

use crate::error::LampResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReportFieldInner {
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportField<T = u32>
where
    T: Into<u32>,
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: Debug,
{
    inner: Option<ReportFieldInner>,
    _phantom: PhantomData<T>,
}

impl<T> Default for ReportField<T>
where
    T: Into<u32>,
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: Debug,
{
    fn default() -> Self {
        Self {
            inner: None,
            _phantom: PhantomData,
        }
    }
}

impl<T> ReportField<T>
where
    T: Into<u32>,
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: Debug,
{
    pub fn set(&mut self, (offset_bits, size_bits): (u32, u32)) -> LampResult<()> {
        assert_eq!(offset_bits % 8, 0);
        assert_eq!(size_bits % 8, 0);

        let offset = offset_bits as usize / 8;
        let size = size_bits as usize / 8;

        assert!(size <= std::mem::size_of::<T>());
        assert!(self.inner.is_none());

        self.inner = Some(ReportFieldInner { offset, size });
        Ok(())
    }

    pub fn set_if_none(&mut self, args: (u32, u32)) -> LampResult<()> {
        if self.inner.is_none() {
            self.set(args)?;
        }

        Ok(())
    }

    pub fn is_some(&self) -> bool {
        self.inner.is_some()
    }

    pub fn size(&self) -> usize {
        self.inner.unwrap().size
    }

    pub fn extract(&self, bytes: &[u8]) -> T {
        let ReportFieldInner { offset, size } = self.inner.unwrap();

        let mut buffer = [0u8; 4];
        buffer[..size].copy_from_slice(&bytes[offset..(offset + size)]);
        u32::from_le_bytes(buffer).try_into().unwrap()
    }

    pub fn write(&self, bytes: &mut [u8], value: T) {
        let ReportFieldInner { offset, size } = self.inner.unwrap();

        let value = value.into().to_le_bytes();
        bytes[offset..(offset + size)].copy_from_slice(&value[..size]);
    }
}

impl<T> AddAssign<usize> for ReportField<T>
where
    T: Into<u32>,
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: Debug,
{
    fn add_assign(&mut self, rhs: usize) {
        self.inner.as_mut().unwrap().offset += rhs;
    }
}
