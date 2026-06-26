use crate::device::{hid_info::HidInfo, iter::DeviceIter};

#[derive(clap::Args, Debug)]
#[command(next_help_heading = "Device selectors")]
pub struct DeviceArgs {
    /// The path name for the device under /dev
    #[arg(long)]
    dev_name: Option<String>,

    /// Device vendor id
    #[arg(long)]
    device_vendor: Option<u32>,

    /// Device product id
    #[arg(long)]
    device_product: Option<u32>,

    /// Device physical address
    #[arg(long)]
    device_phys: Option<String>,

    /// Device unique address
    #[arg(long)]
    device_uniq: Option<String>,

    /// Device name
    #[arg(long)]
    device_name: Option<String>,
}

impl DeviceArgs {
    pub fn dev_name(&self) -> Option<&str> {
        self.dev_name.as_deref()
    }

    pub fn iter(&self) -> anyhow::Result<DeviceIter<'_>> {
        DeviceIter::new(self)
    }

    pub fn satisfies(&self, info: &HidInfo) -> bool {
        if let Some(device_vendor) = &self.device_vendor
            && *device_vendor != info.id.vendor
        {
            return false;
        }

        if let Some(device_product) = &self.device_product
            && *device_product != info.id.product
        {
            return false;
        }

        if let Some(device_phys) = &self.device_phys
            && Some(device_phys) != info.phys.as_ref()
        {
            return false;
        }

        if let Some(device_uniq) = &self.device_uniq
            && Some(device_uniq) != info.uniq.as_ref()
        {
            return false;
        }

        if let Some(device_name) = &self.device_name
            && Some(device_name) != info.name.as_ref()
        {
            return false;
        }

        true
    }
}
