use std::range::RangeInclusive;

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct LampIdArg {
    value: String,
}

impl LampIdArg {
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl From<String> for LampIdArg {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl<'a> IntoIterator for &'a LampIdArg {
    type Item = anyhow::Result<LampIdItem>;
    type IntoIter = LampIdIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LampIdIter::new(&self.value)
    }
}

#[derive(Debug)]
pub struct LampIdIter<'a> {
    source: &'a str,
}

impl Iterator for LampIdIter<'_> {
    type Item = anyhow::Result<LampIdItem>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.source.chars().next()?.is_ascii_digit() {
            return Some(Err(anyhow!("expected number")));
        }

        let start = match self.consume_u32() {
            Ok(s) => s,
            Err(err) => return Some(Err(err)),
        };

        match self.consume_char() {
            Some('.') => {
                if Some('.') != self.consume_char() {
                    return Some(Err(anyhow!("expected '.'")));
                }

                let is_inclusive = self.consume_if('=').is_some();

                match self.source.chars().next() {
                    Some(c) if c.is_ascii_digit() => {
                        let end = match self.consume_u32() {
                            Ok(s) => s,
                            Err(err) => return Some(Err(err)),
                        } - (!is_inclusive) as u32;

                        self.consume_if(',');
                        Some(Ok(LampIdItem::Range((start..=end).into())))
                    }
                    _ => Some(Err(anyhow!("expected number"))),
                }
            }
            None | Some(',') => Some(Ok(LampIdItem::Id(start))),
            Some(c) => Some(Err(anyhow!("unexpected '{c}' after number"))),
        }
    }
}

impl<'a> LampIdIter<'a> {
    fn new(source: &'a str) -> Self {
        Self { source }
    }

    fn consume_char(&mut self) -> Option<char> {
        let char = self.source.chars().next()?;
        self.source = &self.source[char.len_utf8()..];
        Some(char)
    }

    fn consume_if(&mut self, c: char) -> Option<char> {
        if Some(c) == self.source.chars().next() {
            self.consume_char()
        } else {
            None
        }
    }

    fn consume_u32(&mut self) -> anyhow::Result<u32> {
        let len = self.source.chars().take_while(char::is_ascii_digit).count();
        let (slice, source) = self.source.split_at(len);

        self.source = source;

        Ok(slice.parse::<u32>()?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LampIdItem {
    Id(u32),
    Range(RangeInclusive<u32>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_lamp_id() {
        let mut iter = LampIdIter::new("432");

        assert_eq!(iter.next().unwrap().unwrap(), LampIdItem::Id(432));
        assert!(iter.next().is_none());

        let mut iter = LampIdIter::new("8,");

        assert_eq!(iter.next().unwrap().unwrap(), LampIdItem::Id(8));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_invalid_ids() {
        let mut iter = LampIdIter::new("-12");

        assert!(iter.next().unwrap().is_err());

        let mut iter = LampIdIter::new("8,abc");

        assert_eq!(iter.next().unwrap().unwrap(), LampIdItem::Id(8));
        assert!(iter.next().unwrap().is_err());

        let mut iter = LampIdIter::new("8abc");

        assert!(iter.next().unwrap().is_err());

        let mut iter = LampIdIter::new("8 abc");

        assert!(iter.next().unwrap().is_err());
    }

    #[test]
    fn test_multiple_lamp_ids() {
        let mut iter = LampIdIter::new("432,12,18");

        assert_eq!(iter.next().unwrap().unwrap(), LampIdItem::Id(432));
        assert_eq!(iter.next().unwrap().unwrap(), LampIdItem::Id(12));
        assert_eq!(iter.next().unwrap().unwrap(), LampIdItem::Id(18));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_simple_range() {
        let mut iter = LampIdIter::new("12..18");

        assert_eq!(
            iter.next().unwrap().unwrap(),
            LampIdItem::Range((12..=17).into())
        );
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_mixed_ranges() {
        let mut iter = LampIdIter::new("12..18,183,2..=4,181");

        assert_eq!(
            iter.next().unwrap().unwrap(),
            LampIdItem::Range((12..=17).into())
        );
        assert_eq!(iter.next().unwrap().unwrap(), LampIdItem::Id(183));
        assert_eq!(
            iter.next().unwrap().unwrap(),
            LampIdItem::Range((2..=4).into())
        );
        assert_eq!(iter.next().unwrap().unwrap(), LampIdItem::Id(181));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_invalid_ranges() {
        let mut iter = LampIdIter::new("12.18");

        assert!(iter.next().unwrap().is_err());

        let mut iter = LampIdIter::new("12..,4");

        assert!(iter.next().unwrap().is_err());

        let mut iter = LampIdIter::new("12..==3");

        assert!(iter.next().unwrap().is_err());

        let mut iter = LampIdIter::new("..18");

        assert!(iter.next().unwrap().is_err());
    }
}
