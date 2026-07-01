use rgblamp::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut devices = rgblamp::enumerate()?;

    for device in &mut devices {
        assert!(device.lamps().len() >= 3, "needs a device with >=3 lamps");

        // Create a builder.
        let mut builder = device.builder();

        // Set lamps.
        builder.set(0, Color::RED)?;
        builder.set(1, Color::GREEN)?;
        builder.set(2, Color::BLUE)?;

        // Set a range.
        builder.set_range(0..=4, Color::WHITE)?;

        // Finish the query.
        builder.finish()?;
    }

    Ok(())
}
