use std::fs::{self, OpenOptions};

use rgblamp::reports::Reports;

fn main() {
    let data = fs::read("./.ignore/d.bin").unwrap();
    let x = Reports::from_descriptor(&data);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/hidraw1")
        .unwrap();

    let y = x.lamp_array_attrs.send(&mut file);

    dbg!(y);
}
