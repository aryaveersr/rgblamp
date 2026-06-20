use rgblamp::LampArray;

fn main() {
    let lamparrays = LampArray::enumerate();

    for lamparray in lamparrays {
        dbg!(lamparray);
    }
}
