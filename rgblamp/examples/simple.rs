use std::error::Error;

use rgblamp::Color;

fn main() -> Result<(), Box<dyn Error>> {
    // Enumerate all available devices.
    let mut devices = rgblamp::enumerate()?;

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
