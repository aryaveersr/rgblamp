use color::Rgba8;
use log::{error, trace};

use crate::{
    Error, LampArray,
    reports::{
        LampUpdateFlags,
        lamp_multi_update::{LampMultiUpdateParams, LampUpdateItem},
    },
};

/// An update builder to automatically batch multiple lamp updates.
///
/// # Example
///
/// ```ignore
/// let mut builder = lamparray.builder();
///
/// builder.set(0, RED);
/// builder.set(1, GREEN);
/// builder.set(2, BLUE);
///
/// builder.finish(true);
/// ```
#[derive(Debug)]
pub struct LampUpdateBuilder<'a> {
    lamp_array: &'a mut LampArray,
    buffer: Vec<LampUpdateItem>,
    slots: usize,
}

impl<'a> LampUpdateBuilder<'a> {
    pub(crate) fn new(lamp_array: &'a mut LampArray) -> Self {
        let slots = lamp_array.reports.lamp_multi_update.slots() as usize;
        Self {
            lamp_array,
            buffer: Vec::with_capacity(slots),
            slots,
        }
    }

    /// Add an update request to set a lamp to a specific color.
    ///
    /// # Errors
    /// - [`Error::InvalidLampID`]: Lamp ID must be valid, i.e. 0 <= lamp_id < lamp_count.
    pub fn set(&mut self, lamp_id: u32, color: Rgba8) -> crate::Result<&mut Self> {
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

        self.buffer.push(LampUpdateItem { lamp_id, color });
        Ok(self)
    }

    /// Finish any pending update requests and optionally mark this as the last request.
    pub fn finish(self, is_last: bool) -> crate::Result<()> {
        trace!(
            "an update builder finishing with is_last: {is_last} for {}",
            self.lamp_array.dev_name
        );

        self.lamp_array.reports.lamp_multi_update.send(
            &mut self.lamp_array.file,
            LampMultiUpdateParams {
                update_flags: LampUpdateFlags::new(is_last),
                items: &self.buffer,
            },
        )
    }
}
