use std::{fmt::Display, range::RangeIter};

/// A range of u32s bounded inclusively below and exclusively above.
#[derive(Debug, Clone, Copy, Eq)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

impl Range {
    pub const EMPTY: Self = Self::new(0, 0);

    /// Create a new range.
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Range is empty when `self.start >= self.end`.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Returns true if there is atleast one value in the range greater than `value`.
    pub fn exceeds(&self, value: u32) -> bool {
        self.end > value
    }
}

impl IntoIterator for Range {
    type Item = u32;
    type IntoIter = RangeIter<u32>;

    fn into_iter(self) -> Self::IntoIter {
        std::range::Range {
            start: self.start,
            end: self.end,
        }
        .into_iter()
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.is_empty() && other.is_empty())
    }
}

impl<T: Into<u32>> From<std::ops::Range<T>> for Range {
    fn from(value: std::ops::Range<T>) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl From<std::ops::RangeInclusive<u32>> for Range {
    fn from(value: std::ops::RangeInclusive<u32>) -> Self {
        Self {
            start: *value.start(),
            end: value.end() + 1,
        }
    }
}

impl<T: Into<u32>> From<std::range::Range<T>> for Range {
    fn from(value: std::range::Range<T>) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl<T: Into<u32>> From<std::range::RangeInclusive<T>> for Range {
    fn from(value: std::range::RangeInclusive<T>) -> Self {
        Self {
            start: value.start.into(),
            end: value.last.into() + 1,
        }
    }
}
