use std::{fs::File, os::fd::AsRawFd};

use crate::{error::LampResult, utils::info::ReportInfo};

mod ioctl {
    use nix::ioctl_readwrite_buf;
    ioctl_readwrite_buf!(hid_get_feature, b'H', 0x07, u8);
    ioctl_readwrite_buf!(hid_set_feature, b'H', 0x06, u8);
}

pub fn get_feature(file: &mut File, info: &ReportInfo) -> LampResult<Vec<u8>> {
    let mut buffer = vec![0u8; 1 + info.bytes_len()];
    buffer[0] = info.id;

    unsafe {
        ioctl::hid_get_feature(file.as_raw_fd(), &mut buffer)?;
    }

    Ok(buffer)
}

pub fn prep_feature(info: &ReportInfo) -> Vec<u8> {
    let mut buffer = vec![0u8; 1 + info.bytes_len()];
    buffer[0] = info.id;
    buffer
}

pub fn set_feature(file: &mut File, buffer: &mut [u8]) -> LampResult<()> {
    unsafe {
        ioctl::hid_set_feature(file.as_raw_fd(), buffer)?;
    }

    Ok(())
}
