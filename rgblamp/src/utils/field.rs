use std::{fmt::Debug, marker::PhantomData, ops::AddAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ReportField<T = u32>
where
    T: Into<u32>,
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: Debug,
{
    pub offset: usize,
    pub size: usize,
    _phantom: PhantomData<T>,
}

impl<T> ReportField<T>
where
    T: Into<u32>,
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: Debug,
{
    pub fn new(offset_bits: u32, size_bits: u32) -> Self {
        assert_eq!(offset_bits % 8, 0);
        assert_eq!(size_bits % 8, 0);

        let offset = offset_bits as usize / 8;
        let size = size_bits as usize / 8;

        assert!(size <= std::mem::size_of::<T>());

        Self {
            offset,
            size,
            _phantom: PhantomData,
        }
    }

    pub fn is_uninit(&self) -> bool {
        self.offset == 0 && self.size == 0
    }

    pub fn get(&self, bytes: &[u8]) -> T {
        let mut buffer = [0u8; 4];
        buffer[..self.size].copy_from_slice(&bytes[self.offset..(self.offset + self.size)]);
        u32::from_le_bytes(buffer).try_into().unwrap()
    }

    pub fn set(&self, bytes: &mut [u8], value: T) {
        let value = value.into().to_le_bytes();
        bytes[self.offset..(self.offset + self.size)].copy_from_slice(&value[..self.size]);
    }

    pub fn cast_as<V>(self) -> ReportField<V>
    where
        V: Into<u32>,
        V: TryFrom<u32>,
        <V as TryFrom<u32>>::Error: Debug,
    {
        assert!(self.size <= std::mem::size_of::<V>());

        ReportField {
            offset: self.offset,
            size: self.size,
            _phantom: PhantomData,
        }
    }
}

impl<T> AddAssign<usize> for ReportField<T>
where
    T: Into<u32>,
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: Debug,
{
    fn add_assign(&mut self, rhs: usize) {
        self.offset += rhs;
    }
}
