use std::{fs::File, os::fd::AsRawFd};

use crate::{
    error::LampResult,
    reports::utils::{field::Buffer, info::ReportInfo},
};

mod ioctl {
    use nix::ioctl_readwrite_buf;
    ioctl_readwrite_buf!(hid_get_feature, b'H', 0x07, u8);
    ioctl_readwrite_buf!(hid_set_feature, b'H', 0x06, u8);
}

pub fn get_feature(file: &mut File, info: &ReportInfo) -> LampResult<Buffer> {
    let mut buffer = Buffer::new(info);

    unsafe {
        ioctl::hid_get_feature(file.as_raw_fd(), &mut buffer.as_mut())?;
    }

    Ok(buffer)
}

pub fn set_feature(file: &mut File, buffer: &mut Buffer) -> LampResult<()> {
    unsafe {
        ioctl::hid_set_feature(file.as_raw_fd(), buffer.as_mut())?;
    }

    Ok(())
}
