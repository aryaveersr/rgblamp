use std::fs::{self, OpenOptions};

use rgblamp::reports::{
    Reports, UpdateFlags,
    lamp_multi_update::{LampMultiUpdateItem, LampMultiUpdateParams},
};

fn main() {
    let data = fs::read("./.ignore/d.bin").unwrap();
    let x = Reports::from_descriptor(&data);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/hidraw4")
        .unwrap();

    x.lamp_multi_update.send(
        &mut file,
        LampMultiUpdateParams {
            lamp_update_flags: UpdateFlags::new(true),
            items: &[LampMultiUpdateItem {
                lamp_id: 0,
                red_update_channel: 0,
                green_update_channel: 127,
                blue_update_channel: 255,
                intensity_update_channel: 255,
            }],
        },
    );

    // x.lamp_attrs_request.send(&mut file, 0);
    // let y = x.lamp_attrs_response.send(&mut file);

    // dbg!(y);
}
