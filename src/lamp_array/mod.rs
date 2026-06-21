use std::{
    fs::{self, File, OpenOptions},
    ops::RangeInclusive,
    path::{Path, PathBuf},
    time::Duration,
};

use color::Rgba8;
use log::{error, trace};
use serde::Serialize;

use crate::{
    error::{Error, LampResult},
    parser::ReportDescriptorParser,
    reports::{
        LampUpdateFlags, Reports, lamp_multi_update::LampMultiUpdateParams,
        lamp_range_update::LampRangeUpdateParams,
    },
};

#[derive(Debug)]
pub struct LampArray {
    id: usize,
    path: PathBuf,
    file: File,
    reports: Reports,
    min_update_interval: Duration,
    lamps: Vec<LampAttrs>,
}

impl LampArray {
    pub fn enumerate() -> LampResult<Vec<Self>> {
        trace!("Enumerating LampArray devices");

        let mut lamparrays = Vec::new();
        let mut entries: Vec<_> = fs::read_dir("/sys/class/hidraw")
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();

        entries.sort_by_key(|dir| dir.file_name());

        let mut id_counter = 0;

        for entry in entries {
            let descriptor = entry.path().join("device/report_descriptor");
            let bytes = fs::read(descriptor).unwrap();
            let parser = ReportDescriptorParser::new(&bytes);
            let path = PathBuf::from("/dev").join(entry.path().file_name().unwrap());

            for reports in parser {
                trace!("Found a device at {path:?} (id {id_counter})");

                let reports = reports?;

                lamparrays.push(Self::new(id_counter, path.clone(), reports)?);
                id_counter += 1;
            }
        }

        Ok(lamparrays)
    }

    fn new(id: usize, path: PathBuf, reports: Reports) -> LampResult<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();

        let attrs = reports.lamp_array_attrs.get(&mut file)?;
        let mut lamps = Vec::with_capacity(attrs.lamp_count as usize);

        trace!("Received LampArray attributes: {attrs:?}");

        if attrs.lamp_count == 0 {
            error!("Device id {id} has no lamps");
            return Err(Error::NoLamps);
        }

        reports.lamp_attrs_request.send(&mut file, 0)?;
        for lamp_id in 0..attrs.lamp_count {
            let attrs = reports.lamp_attrs_response.get(&mut file)?;

            trace!("Received Lamp attributes for lamp {lamp_id}: {attrs:?}");

            if !attrs.is_programmable {
                // TODO
                error!("Lamp {lamp_id} is not programmable");
                return Err(Error::unsupported("non-programmable lamp"));
            }

            lamps.push(attrs);
        }

        Ok(Self {
            id,
            path,
            file,
            reports,
            min_update_interval: Duration::from_micros(attrs.min_update_interval_us as u64),
            lamps,
        })
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn min_update_interval(&self) -> Duration {
        self.min_update_interval
    }

    pub fn lamps(&self) -> &[LampAttrs] {
        &self.lamps
    }

    pub fn set_auto_mode(&mut self, auto_mode: bool) -> LampResult<()> {
        trace!("Setting auto mode to '{auto_mode}' for device {}", self.id);

        self.reports
            .lamp_array_control
            .send(&mut self.file, auto_mode)
    }

    pub fn set_lamp(&mut self, lamp_id: u32, color: Rgba8) -> LampResult<()> {
        trace!(
            "Setting lamp {lamp_id} to color '{color}' for device {}",
            self.id
        );

        if lamp_id >= self.lamps.len() as u32 {
            error!("Lamp id {lamp_id} was invalid (out of range)");
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

    pub fn set_all_lamps(&mut self, color: Rgba8) -> LampResult<()> {
        trace!(
            "Setting all lamps to color '{color}' for device {}",
            self.id
        );

        self.reports.lamp_range_update.send(
            &mut self.file,
            LampRangeUpdateParams {
                lamp_ids: 0..=(self.lamps.len() as u32 - 1),
                update_flags: LampUpdateFlags::new(true),
                color,
            },
        )
    }

    pub fn set_lamps_range(
        &mut self,
        lamp_ids: RangeInclusive<u32>,
        color: Rgba8,
        is_last: bool,
    ) -> LampResult<()> {
        trace!(
            "Setting all lamps in range {lamp_ids:?} to color '{color}' for device {}",
            self.id
        );
        trace!("Is this is last in a batch: {is_last}");

        if *lamp_ids.end() >= self.lamps.len() as u32 {
            error!("One or more lamp ids in range {lamp_ids:?} is invalid");
            return Err(Error::InvalidLampID);
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

    pub fn set_multiple_lamps(
        &mut self,
        items: &[LampUpdateItem],
        is_last: bool,
    ) -> LampResult<()> {
        trace!("Setting multiple lamps for device {}", self.id);
        trace!("{items:?}");
        trace!("Is this is last in a batch: {is_last}");

        for item in items {
            if item.lamp_id >= self.lamps.len() as u32 {
                error!("Lamp id {} was invalid (out of range)", item.lamp_id);
                return Err(Error::InvalidLampID);
            }
        }

        let slots = self.reports.lamp_multi_update.slots() as usize;
        let last_idx = items.len().div_ceil(slots);

        for (idx, chunk) in items.chunks(slots).enumerate() {
            self.reports.lamp_multi_update.send(
                &mut self.file,
                LampMultiUpdateParams {
                    update_flags: LampUpdateFlags::new(is_last && last_idx == idx),
                    items: chunk,
                },
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct LampAttrs {
    pub lamp_id: u32,
    pub update_latency: Duration,
    pub is_programmable: bool,

    pub red_level_count: u32,
    pub green_level_count: u32,
    pub blue_level_count: u32,
    pub intensity_level_count: u32,
}

#[derive(Debug)]
pub struct LampUpdateItem {
    pub lamp_id: u32,
    pub color: Rgba8,
}
