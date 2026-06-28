<div style="text-align: center;" align="center">

# `rgblamp`

### A library to control HID LampArray devices on Linux.

[![MIT License](https://img.shields.io/crates/l/rgblamp?style=for-the-badge)](https://github.com/aryaveersr/rgblamp/blob/main/LICENSE)
[![Docs](https://img.shields.io/docsrs/rgblamp?style=for-the-badge)](https://docs.rs/rgblamp/latest/rgblamp/)

</div>

## About

LampArray is the protocol used by Windows Dynamic Lighting to control RGB devices. As such, its the closest we have to a standard for RGB lighting and is currently implemented by many devices. This library allows you to interface with these devices.

## Setup

For instructions on how to setup permissions, refer to the [workspace README](https://github.com/aryaveersr/rgblamp).

## Examples

```rust
use std::{error::Error, fs};

use rgblamp::ReportDescriptorParser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut devices = Vec::new();

    // Scan the `/sys/class/hidraw` directory for lamparray devices.
    // You can filter devices according to their properties such as vendor and product ids here.
    for entry in fs::read_dir("/sys/class/hidraw")? {
        let entry = entry?;

        let file_name = entry.file_name();
        let dev_name = file_name.to_str().unwrap().to_owned();

        let contents = fs::read(entry.path().join("device/report_descriptor"))?;
        let mut parser = ReportDescriptorParser::new(&contents);

        while let Some(device) = parser.next(&dev_name)? {
            devices.push(device);
        }
    }

    // Print devnames for all devices.
    for device in &devices {
        println!("{}", device.dev_name());
    }

    for device in &mut devices {
        // Disable auto mode (transfer control to host device);
        device.set_auto_mode(false)?;
        // Set all devices to blue.
        device.set_all_lamps(color::palette::css::BLUE.to_rgba8())?;
    }

    Ok(())
}
```

More examples can be found in the [examples directory](https://github.com/aryaveersr/rgblamp/tree/main/rgblamp/examples). Run examples using:

```sh
cargo run -p rgblamp --example EXAMPLE_NAME_HERE
```
