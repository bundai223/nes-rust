use std::{fs, u8};
use std::io::{BufReader, Read};

const NES_HEADER_SIGNATURE_SIZE: usize = 4;
const NES_HEADER_SIZE: usize = 6;
const NES_PROGRAM_ROM_SIZE: usize = 16 * 1024 * 1024;
const NES_CHARACTER_ROM_SIZE: usize = 8 * 1024 * 1024;
const NES_ROM_SIZE: usize           = NES_HEADER_SIZE + NES_PROGRAM_ROM_SIZE + NES_CHARACTER_ROM_SIZE;

fn allocate(size: usize) -> Box<[u8]> {
    let mut tmpvec = Vec::<u8>::with_capacity(size);
    unsafe {
        tmpvec.set_len(size);
    }
    return tmpvec.into_boxed_slice();
}

fn header_dump(rom: Box<[u8]>) {
    println!("Header Dump");
    for i in 0..NES_HEADER_SIGNATURE_SIZE {
      print!("{:X} ", (*rom)[i]);
    }
    println!("");
    println!("Size of PRG ROM in 16 KB units : {:X} ", (*rom)[NES_HEADER_SIGNATURE_SIZE + 0]);
    println!("Size of CHR ROM in  8 KB units : {:X} ", (*rom)[NES_HEADER_SIGNATURE_SIZE + 1]);
}

fn main() {
    let file = fs::File::open("rom/sample1/sample1.nes").unwrap();
    let filesize = file.metadata().unwrap().len();
    let mut reader = BufReader::new(file);

    let mut rom_data: Box<[u8]> = allocate(filesize as usize);

    reader.read_exact(&mut (*rom_data)).unwrap();

    // println!("File dumo {}", filesize);
    header_dump(rom_data);

    println!("");
}
