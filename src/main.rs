use std::fs;

use rgblamp::reports::Reports;

fn main() {
    let data = fs::read("./.ignore/d.bin").unwrap();
    let x = Reports::from_descriptor(&data);

    dbg!(x);
}
