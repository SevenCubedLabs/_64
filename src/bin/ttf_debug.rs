use std::{convert::TryInto, fs::File, io::Read};
use underscore_64::ttf::Font;

fn main() {
    let hack: Font = File::open("underscore_64/src/assets/Hack-Regular.ttf")
        .expect("open Hack-Regular.ttf")
        .bytes()
        .collect::<Result<Vec<u8>, std::io::Error>>()
        .expect("read Hack-Regular.ttf")
        .as_slice()
        .try_into()
        .expect("parse Hack-Regular.ttf");

    println!("Parsed Hack-Regular.ttf!:\n{:?}", hack);
}
