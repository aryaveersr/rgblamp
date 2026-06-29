use rgblamp::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enumerate all available devices.
    let mut devices = rgblamp::enumerate()?;

    for device in &mut devices {
        // Print devnames for all devices.
        println!("{}", device.dev_name());
        // Disable auto mode (transfer control to host device);
        device.set_auto_mode(false)?;
        // Set all devices to blue.
        device.set_all_lamps(Color::BLUE)?;
    }

    Ok(())
}
