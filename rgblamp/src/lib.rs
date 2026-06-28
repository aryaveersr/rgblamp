#![doc = include_str!("../README.md")]

mod builder;
mod color;
mod error;
mod lamp_array;
mod parser;
mod reports;

pub use builder::LampUpdateBuilder;
pub use color::Color;
pub use error::*;
pub use lamp_array::*;
pub use parser::ReportDescriptorParser;

/// Enumerate all lamparray devices.
///
/// # Example
///
/// ```
/// let mut devices = rgblamp::enumerate()?;
/// for lamparray in &mut devices {
///     lamparray.set_auto_mode(false);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn enumerate() -> Result<Vec<LampArray>> {
    let mut devices = Vec::new();

    for entry in std::fs::read_dir("/sys/class/hidraw")? {
        let entry = entry?;

        let file_name = entry.file_name();
        let dev_name = file_name.to_str().unwrap().to_owned();

        let contents = std::fs::read(entry.path().join("device/report_descriptor"))?;
        let mut parser = ReportDescriptorParser::new(&contents);

        while let Some(lamp_array) = parser.next(&dev_name)? {
            devices.push(lamp_array);
        }
    }

    Ok(devices)
}
