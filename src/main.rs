use std::{fs, mem};
use std::io::{BufReader, Read};

fn main() {
    let mut f = BufReader::new(fs::File::open("rom/sample1/sample1.nes").unwrap());
    let mut b: [ u8; 40976] = unsafe { mem::uninitialized() };

    f.read_exact(&mut b).unwrap();

    println!("Hello, world!");
}
