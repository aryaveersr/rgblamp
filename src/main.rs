use color::palette::css;
use rgblamp::LampArray;

fn main() {
    let mut lamparray = LampArray::enumerate().pop().unwrap();

    lamparray.set_color_all(css::DARK_RED);
    lamparray.set_auto_mode(false);
}
