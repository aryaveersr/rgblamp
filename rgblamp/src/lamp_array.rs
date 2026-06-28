//! References:
//!
//! HID Spec: <https://www.usb.org/document-library/device-class-definition-hid-111>
//! HUT:      <https://usb.org/document-library/hid-usage-tables-14>
//!
//! The HUT (HID Usage Tables) document has the information for the LampArray interface
//! under Section 26: Lighting and Illumination Page.

use std::{
    fs::{File, OpenOptions},
    ops::RangeInclusive,
    time::Duration,
};

use log::{error, trace};

use crate::{
    Color, LampUpdateBuilder,
    error::Error,
    reports::{LampUpdateFlags, Reports, lamp_range_update::LampRangeUpdateParams},
};

/// A LampArray device. A single physical device can expose multiple LampArray devices.
/// Use [`crate::enumerate`] to find all lamparray devices or use [`crate::ReportDescriptorParser`] to manually create new instances of this struct.
///
/// # Example
///
/// ```
/// let mut devices = rgblamp::enumerate()?;
/// for lamparray in &mut devices {
///     lamparray.set_auto_mode(false);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct LampArray {
    pub(crate) dev_name: String,
    pub(crate) file: File,
    pub(crate) reports: Reports,

    min_update_interval: Duration,
    pub(crate) lamps: Vec<LampAttrs>,
}

impl LampArray {
    /// The dev name of the device of this lamparray.
    pub fn dev_name(&self) -> &str {
        &self.dev_name
    }

    /// The minimum interval required between two updates.
    /// Each update can consist of multiple requests where only the last request has `is_last` as `true`.
    ///
    /// # Warning
    /// Not waiting atleast this duration between two updates can possibly harm the device.
    pub fn min_update_interval(&self) -> Duration {
        self.min_update_interval
    }

    /// Get attributes of all the lamps for this device.
    /// See [`LampAttrs`] for more information about these attributes.
    pub fn lamps(&self) -> &[LampAttrs] {
        &self.lamps
    }

    /// Enable or disable autonomous mode for this device.
    ///
    /// This decides who controls the lamps for this device. When its enabled, the device runs its inbuilt effects. Disabling it transfers control to the host and is necessary for other set commands.
    pub fn set_auto_mode(&mut self, auto_mode: bool) -> crate::Result<()> {
        trace!("setting auto mode to '{auto_mode}' for {}", self.dev_name);

        self.reports
            .lamp_array_control
            .send(&mut self.file, auto_mode)
    }

    /// Set a particular lamp to a specific color.
    ///
    /// # Errors
    /// - [`Error::InvalidLampID`]: Lamp ID must be valid, i.e. 0 <= lamp_id < lamp_count.
    pub fn set_lamp(&mut self, lamp_id: u32, color: impl Into<Color>) -> crate::Result<()> {
        let color = color.into();

        trace!(
            "setting lamp {lamp_id} to color '{color}' for {}",
            self.dev_name
        );

        if lamp_id >= self.lamps.len() as u32 {
            error!(
                "lampid {lamp_id} was invalid. number of lamps is {}",
                self.lamps.len()
            );
            return Err(Error::InvalidLampID);
        }

        self.reports.lamp_range_update.send(
            &mut self.file,
            LampRangeUpdateParams {
                lamp_ids: lamp_id..=lamp_id,
                update_flags: LampUpdateFlags::new(true),
                color,
            },
        )
    }

    /// Set all lamps to a specific color.
    pub fn set_all_lamps(&mut self, color: impl Into<Color>) -> crate::Result<()> {
        let color = color.into();

        trace!("setting all lamps to color '{color}' for {}", self.dev_name);

        self.reports.lamp_range_update.send(
            &mut self.file,
            LampRangeUpdateParams {
                lamp_ids: 0..=(self.lamps.len() as u32 - 1),
                update_flags: LampUpdateFlags::new(true),
                color,
            },
        )
    }

    /// Set all lamps in a range to a specific color.
    ///
    /// # Errors
    /// - [`Error::InvalidLampID`]: Lamp IDs must be valid, i.e. 0 <= lamp_ids.end() < lamp_count.
    /// - [`Error::EmptyLampIDRange`]: Range must not be empty.
    pub fn set_lamp_range(
        &mut self,
        lamp_ids: RangeInclusive<u32>,
        color: impl Into<Color>,
        is_last: bool,
    ) -> crate::Result<()> {
        let color = color.into();

        trace!(
            "setting all lamps in range {lamp_ids:?} to color '{color}' for {}",
            self.dev_name
        );
        trace!("is this is last in a batch: {is_last}");

        if *lamp_ids.end() >= self.lamps.len() as u32 {
            error!(
                "lampid range '{lamp_ids:?}' was invalid. number of lamps is {}",
                self.lamps.len()
            );
            return Err(Error::InvalidLampID);
        }

        if lamp_ids.is_empty() {
            error!("lampid range {lamp_ids:?} is empty");
            return Err(Error::EmptyLampIDRange);
        }

        self.reports.lamp_range_update.send(
            &mut self.file,
            LampRangeUpdateParams {
                lamp_ids,
                color,
                update_flags: LampUpdateFlags::new(is_last),
            },
        )
    }

    /// Create an update builder to automatically batch multiple lamp updates.
    /// See [`LampUpdateBuilder`] for more information.
    pub fn builder(&mut self) -> LampUpdateBuilder<'_> {
        trace!("creating builder for {}", self.dev_name);

        LampUpdateBuilder::new(self)
    }

    pub(crate) fn new(dev_name: impl Into<String>, reports: Reports) -> crate::Result<Self> {
        let dev_name = dev_name.into();

        trace!("creating a new lamparray from /dev/{dev_name}");

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("/dev/{dev_name}"))?;

        let attrs = reports.lamp_array_attrs.get(&mut file)?;
        let mut lamps = Vec::with_capacity(attrs.lamp_count as usize);

        trace!("received lamparray attributes: {attrs:?}");

        if attrs.lamp_count == 0 {
            error!("Device has no lamps");
            return Err(Error::NoLamps);
        }

        reports.lamp_attrs_request.send(&mut file, 0)?;
        for lamp_id in 0..attrs.lamp_count {
            let attrs = reports.lamp_attrs_response.get(&mut file)?;

            trace!("received lamp attributes for lamp {lamp_id}: {attrs:?}");

            if !attrs.programmable {
                // TODO
                error!("lamp {lamp_id} is not programmable");
                return Err(Error::unsupported("non-programmable lamp"));
            }

            lamps.push(attrs);
        }

        Ok(Self {
            dev_name,
            file,
            reports,
            min_update_interval: Duration::from_micros(attrs.min_update_interval_us as u64),
            lamps,
        })
    }
}

/// Attributes for a single lamp.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LampAttrs {
    pub lamp_id: u32,

    /// Expected latency for this lamp to reflect an update.
    pub update_latency: Duration,

    /// Whether this lamp is programmable (color can be changed) or fixed.
    /// See section 26.9 (Color Attributes) of the HUT.
    ///
    /// If the lamp is `Fixed`, the RGB level count attributes represent the fixed color.
    pub programmable: bool,

    /// Maximum value for red color channel.
    /// If this lamp is fixed (see [`LampAttrs::programmable`]), it represents the red color value of the fixed color.
    pub red_level_count: u32,

    /// Maximum value for green color channel.
    /// If this lamp is fixed (see [`LampAttrs::programmable`]), it represents the green color value of the fixed color.
    pub green_level_count: u32,

    /// Maximum value for blue color channel.
    /// If this lamp is fixed (see [`LampAttrs::programmable`]), it represents the blue color value of the fixed color.
    pub blue_level_count: u32,

    /// Maximum value for intensity color channel.
    pub intensity_level_count: u32,
}
