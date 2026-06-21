use std::{
    fs::{self, File, OpenOptions},
    ops::RangeInclusive,
    path::PathBuf,
    time::Duration,
};

use color::Rgba8;

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
    file: File,
    reports: Reports,
    array_attrs: LampArrayAttrs,
    lamp_attrs: Vec<LampAttrs>,
}

impl LampArray {
    pub fn enumerate() -> LampResult<Vec<Self>> {
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
            let device_path = PathBuf::from("/dev").join(entry.path().file_name().unwrap());

            for reports in parser {
                let reports = reports?;
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&device_path)
                    .unwrap();

                lamparrays.push(Self::new(id_counter, file, reports)?);
                id_counter += 1;
            }
        }

        Ok(lamparrays)
    }

    fn new(id: usize, mut file: File, reports: Reports) -> LampResult<Self> {
        let array_attrs = reports.lamp_array_attrs.get(&mut file)?;
        let mut lamp_attrs = Vec::with_capacity(array_attrs.lamp_count as usize);

        if array_attrs.lamp_count == 0 {
            return Err(Error::NoLamps);
        }

        reports.lamp_attrs_request.send(&mut file, 0)?;
        for _ in 0..array_attrs.lamp_count {
            let attrs = reports.lamp_attrs_response.get(&mut file)?;

            if !attrs.is_programmable {
                // TODO
                return Err(Error::unsupported("non-programmable lamp"));
            }

            lamp_attrs.push(attrs);
        }

        Ok(Self {
            id,
            file,
            reports,
            array_attrs,
            lamp_attrs,
        })
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn lamp_count(&self) -> u32 {
        self.array_attrs.lamp_count
    }

    pub fn min_update_interval(&self) -> Duration {
        Duration::from_micros(self.array_attrs.min_update_interval_us as u64)
    }

    pub fn lamps(&self) -> &[LampAttrs] {
        &self.lamp_attrs
    }

    pub fn set_auto_mode(&mut self, auto_mode: bool) -> LampResult<()> {
        self.reports
            .lamp_array_control
            .send(&mut self.file, auto_mode)
    }

    pub fn set_lamp(&mut self, lamp_id: u32, color: Rgba8) -> LampResult<()> {
        if lamp_id >= self.lamp_count() {
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
        self.reports.lamp_range_update.send(
            &mut self.file,
            LampRangeUpdateParams {
                lamp_ids: 0..=(self.array_attrs.lamp_count - 1),
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
        if *lamp_ids.end() >= self.lamp_count() {
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
        for item in items {
            if item.lamp_id >= self.lamp_count() {
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

#[derive(Debug)]
pub struct LampArrayAttrs {
    pub lamp_count: u32,
    pub min_update_interval_us: u32,
}

#[derive(Debug)]
pub struct LampAttrs {
    pub lamp_id: u32,
    pub update_latency_us: u32,
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
