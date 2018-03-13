use std::{fs, mem};
use std::io::{BufReader, Read};

const NES_HEADER_SIZE: usize = 4;
const NES_PROGRAM_ROM_SIZE: usize = 16 * 1024 * 1024;
const NES_CHARACTER_ROM_SIZE: usize = 8 * 1024 * 1024;

fn main() {
    let mut f = BufReader::new(fs::File::open("rom/sample1/sample1.nes").unwrap());
    let mut header: [ u8; NES_HEADER_SIZE] = unsafe { mem::uninitialized() };
    // let mut programrom: [ u8; NES_PROGRAM_ROM_SIZE] = unsafe { mem::uninitialized() };
    // let mut charactorrom: [ u8; NES_CHARACTER_ROM_SIZE] = unsafe { mem::uninitialized() };

    f.read_exact(&mut header).unwrap();

    println!("Header Dump");
    for i in 0..NES_HEADER_SIZE {
      print!("{:X} ", header[i]);
    }
    println!("");
}
