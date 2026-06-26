use std::{fmt::Debug, marker::PhantomData, ops::AddAssign};

use crate::{
    error::{Error, LampResult},
    reports::utils::info::ReportInfo,
};

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
        if !offset_bits.is_multiple_of(8) || !size_bits.is_multiple_of(8) {
            // TODO
            return Err(Error::unsupported("only byte aligned fields are supported"));
        }

        let offset = offset_bits as usize / 8;
        let size = size_bits as usize / 8;

        if size > std::mem::size_of::<T>() {
            return Err(Error::parser("size of field is too large to store"));
        }

        if self.inner.is_some() {
            return Err(Error::parser("field was already defined"));
        }

        self.inner = Some(ReportFieldInner { offset, size });
        Ok(())
    }

    pub fn set_if_none(&mut self, args: (u32, u32)) -> LampResult<()> {
        if self.inner.is_none() {
            self.set(args)?;
        }

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.inner.unwrap().size
    }

    pub fn validate(&self, usage: &'static str) -> LampResult<()> {
        match self.inner {
            Some(_) => Ok(()),
            None => Err(Error::parser(format!("missing usage for {usage}"))),
        }
    }

    pub fn extract(&self, buffer: &Buffer) -> T {
        let ReportFieldInner { offset, size } = self.inner.unwrap();

        let mut value = [0u8; 4];
        value[..size].copy_from_slice(&buffer.body()[offset..(offset + size)]);
        u32::from_le_bytes(value).try_into().unwrap()
    }

    pub fn write(&self, buffer: &mut Buffer, value: T) {
        let ReportFieldInner { offset, size } = self.inner.unwrap();

        let value = value.into().to_le_bytes();
        buffer.body_mut()[offset..(offset + size)].copy_from_slice(&value[..size]);
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

    fn body(&self) -> &[u8] {
        &self.bytes[1..]
    }

    fn body_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[1..]
    }

    pub(super) fn as_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}
