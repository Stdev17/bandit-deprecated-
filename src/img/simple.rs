use std::time::{Instant};
use std::fs;
use std::io::{BufReader, Read, BufWriter, Write};

pub fn main() {
    let mut source = BufReader::with_capacity(4096, fs::File::open("resources/kokkoro2.png").unwrap());
    let mut buf: Vec<u8> = Vec::new();
    let now = Instant::now();

    source.read_to_end(&mut buf);
    println!("{}", now.elapsed().as_micros());

    let mut dest = BufWriter::with_capacity(4096, fs::File::create("test.png").unwrap());
    dest.write_all(&buf).unwrap();
    println!("{}", now.elapsed().as_micros());
}