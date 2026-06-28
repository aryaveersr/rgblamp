use std::{error::Error, fs};

use rgblamp::{Color, ReportDescriptorParser};

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
        device.set_all_lamps(Color::BLUE)?;
    }

    Ok(())
}
