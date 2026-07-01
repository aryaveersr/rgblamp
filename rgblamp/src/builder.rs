use log::{error, trace};

use crate::{
    Color, Error, LampArray, LampUpdateItem,
    reports::{
        LampUpdateFlags, lamp_multi_update::LampMultiUpdateParams,
        lamp_range_update::LampRangeUpdateParams,
    },
};

/// An update builder to automatically batch multiple lamp updates.
///
/// # Example
///
/// ```
/// # use rgblamp::Color;
/// # let lamparray = &mut rgblamp::enumerate()?[0];
/// let mut builder = lamparray.builder();
///
/// builder.set(0, Color::RED);
/// builder.set(1, Color::GREEN);
/// builder.set(2, Color::BLUE);
///
/// builder.set_range(0..=4, Color::WHITE);
///
/// builder.finish();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct LampUpdateBuilder<'a> {
    lamp_array: &'a mut LampArray,
    // Buffer of pending updates. This is never empty after set() is called atleast once.
    buffer: Vec<LampUpdateItem>,
    slots: usize,
    // A pending range update incase its the last request, in which case it is finalized in finish().
    range: Option<(std::range::RangeInclusive<u32>, Color)>,
}

impl<'a> LampUpdateBuilder<'a> {
    pub(crate) fn new(lamp_array: &'a mut LampArray) -> Self {
        let slots = lamp_array.reports.lamp_multi_update.slots() as usize;
        Self {
            lamp_array,
            buffer: Vec::with_capacity(slots),
            slots,
            range: None,
        }
    }

    /// Add an update request to set a lamp to a specific color.
    ///
    /// # Errors
    /// - [`Error::InvalidLampID`]: Lamp ID must be valid, i.e. 0 <= lamp_id < lamp_count.
    pub fn set(&mut self, lamp_id: u32, color: impl Into<Color>) -> crate::Result<&mut Self> {
        let color = color.into();

        trace!(
            "an update builder is setting lamp {lamp_id} to color '{color}' for {}",
            self.lamp_array.dev_name
        );

        if lamp_id >= self.lamp_array.lamps.len() as u32 {
            error!(
                "lampid {lamp_id} was invalid. number of lamps is {}",
                self.lamp_array.lamps.len()
            );
            return Err(Error::InvalidLampID);
        }

        if self.buffer.len() == self.slots {
            self.lamp_array.reports.lamp_multi_update.send(
                &mut self.lamp_array.file,
                LampMultiUpdateParams {
                    update_flags: LampUpdateFlags::new(false),
                    items: &self.buffer,
                },
            )?;
            self.buffer.clear();
        }

        if let Some((lamp_ids, color)) = self.range.take() {
            self.lamp_array.reports.lamp_range_update.send(
                &mut self.lamp_array.file,
                LampRangeUpdateParams {
                    lamp_ids,
                    update_flags: LampUpdateFlags::new(false),
                    color,
                },
            )?;
        }

        self.buffer.push(LampUpdateItem { lamp_id, color });
        Ok(self)
    }

    /// Add an update request to set a range of lamps to a specific color.
    ///
    /// # Errors
    /// - [`Error::InvalidLampID`]: Lamp IDs must be valid, i.e. 0 <= lamp_ids.end() < lamp_count.
    /// - [`Error::EmptyLampIDRange`]: Range must not be empty.
    pub fn set_range(
        &mut self,
        lamp_ids: impl Into<std::range::RangeInclusive<u32>>,
        color: impl Into<Color>,
    ) -> crate::Result<()> {
        let lamp_ids = lamp_ids.into();
        let color = color.into();

        trace!(
            "an update builder is setting all lamps in range {lamp_ids:?} to color '{color}' for {}",
            self.lamp_array.dev_name
        );

        if lamp_ids.last >= self.lamp_array.lamps.len() as u32 {
            error!(
                "lampid range {lamp_ids:?} was invalid. number of lamps is {}",
                self.lamp_array.lamps.len()
            );
            return Err(Error::InvalidLampID);
        }

        if lamp_ids.is_empty() {
            error!("lampid range {lamp_ids:?} is empty");
            return Err(Error::EmptyLampIDRange);
        }

        if let Some((lamp_ids, color)) = self.range.replace((lamp_ids, color)) {
            self.lamp_array.reports.lamp_range_update.send(
                &mut self.lamp_array.file,
                LampRangeUpdateParams {
                    lamp_ids,
                    update_flags: LampUpdateFlags::new(false),
                    color,
                },
            )?;
        }

        Ok(())
    }

    /// Finish any pending update requests.
    pub fn finish(mut self) -> crate::Result<()> {
        trace!(
            "an update builder finishing for {}",
            self.lamp_array.dev_name
        );

        if let Some((lamp_ids, color)) = self.range.take() {
            self.lamp_array.reports.lamp_range_update.send(
                &mut self.lamp_array.file,
                LampRangeUpdateParams {
                    lamp_ids,
                    update_flags: LampUpdateFlags::new(true),
                    color,
                },
            )?;
            return Ok(());
        }

        if self.buffer.is_empty() {
            trace!("the update builder was empty");
            return Ok(());
        }

        if self.buffer.len() == 1 {
            let id = self.buffer[0].lamp_id;
            self.lamp_array.reports.lamp_range_update.send(
                &mut self.lamp_array.file,
                LampRangeUpdateParams {
                    lamp_ids: (id..=id).into(),
                    update_flags: LampUpdateFlags::new(true),
                    color: self.buffer[0].color,
                },
            )
        } else {
            self.lamp_array.reports.lamp_multi_update.send(
                &mut self.lamp_array.file,
                LampMultiUpdateParams {
                    update_flags: LampUpdateFlags::new(true),
                    items: &self.buffer,
                },
            )
        }
    }
}
