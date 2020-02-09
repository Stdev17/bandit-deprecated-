
use std::time::{Instant};
//use image::GenericImageView;
//use std::{fs, mem};
use std::{fs, str};
use std::io::{BufReader, Read, BufWriter, Write};
use std::string::String;
use crate::algo::grayscale::set;
/*
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
*/

struct Chunk {
    length: i32,  // chunk data length
    c_type: String,// chunk type
    data: Vec<u8>,// chunk data
    crc32:Vec<u8> // CRC32 of chunk data
}

struct PNG {
    width: i32,
    height: i32,
    bit_depth: i32,
    color_type: i32,
    compression: i32,
    filter: i32,
    interlace: i32,
    chunks: Vec<Chunk>,
    num_chunks: i32,
    dats: Vec<i32>
}

// u_int32_to_int converts a 4 byte big-endian buffer to int.
fn u_int32_to_int(buf: &[u8]) -> i32 {
    let mut res: i32 = 0;
    for i in 0..4 {
        res += 2i32.pow(i*8)*(buf[3-i as usize] as i32);
    }
    return res;
}

pub fn readpng() {
    let now = Instant::now();
    let mut source = BufReader::new(fs::File::open("resources/kokkoro2.png").unwrap());
    //let mut b: [u8; 4] = unsafe { mem::MaybeUninit::uninit().assume_init() };
    let mut header: Vec<u8> = vec![0; 8];
    source.read_exact(&mut header).unwrap();
    /*
    let mut m: usize;
    
    loop {
        m = source.read_until(b' ', &mut b).expect("");
        if m == 0 {
            break
        }
    }
    */
    let mut ihdr = Chunk::new();
    Chunk::populate(&mut ihdr, &mut source);

    let mut png = PNG::new();
    PNG::parse_ihdr(&mut png, &mut ihdr);
    PNG::add(&mut png, ihdr);

    loop {
        let mut ch = Chunk::new();
        Chunk::populate(&mut ch, &mut source);
        let chk = &ch.c_type.clone();
        //println!("{}", &chk);
        PNG::add(&mut png, ch);
        if chk == "IEND" {
            break;
        }
    }
    
    //println!("{:?}", f);
    //println!("{}", &png.chunks.len());
    println!("{}", now.elapsed().as_micros());

    let dest = BufWriter::new(fs::File::create("test.png").unwrap());
}

impl Chunk {
    // Populate will read bytes from the reader and populate a chunk.
    fn populate(&mut self, s: &mut BufReader<std::fs::File>) {
        
        // Four byte buffer.
        let mut buf: Vec<u8> = vec![0; 4];

        // Read first four bytes == chunk length.
        s.read_exact(&mut buf).unwrap();

        //println!("{:?}", &buf);
        // Convert bytes to int.
        // c.length = int(binary.BigEndian.Uint32(buf))
        self.length = u_int32_to_int(&buf[0..4]);

        let mut buf2: Vec<u8> = vec![0; 4];
        // Read second four bytes == chunk type.
        s.read_exact(&mut buf2).unwrap();
        self.c_type = str::from_utf8(&buf2).unwrap().to_string();

        // Read chunk data.
        
        let mut tmp: Vec<u8> = vec![0; self.length as usize];
        s.read_exact(&mut tmp).unwrap();
        if self.c_type == "IDAT" {
            self.data = set(tmp);
        } else {
            self.data = tmp;
        }

        // Read CRC32 hash
        let mut buf3: Vec<u8> = vec![0; 4];
        // Read second four bytes == chunk type.
        s.read_exact(&mut buf3).unwrap();
        // We don't really care about checking the hash.
        self.crc32 = buf3;
    }

    fn new() -> Chunk {
        Chunk { length: 0, c_type: "".to_string(), data: Vec::new(), crc32: Vec::new() }
    }
}

impl PNG {
    fn add(&mut self, c: Chunk) {
        self.chunks.push(c);
        self.num_chunks += 1;
    }
    fn parse_ihdr (&mut self, ihdr: &mut Chunk) {
        const IHDR_LENGTH: i32 = 13;
        if ihdr.length != IHDR_LENGTH {
            return;
        }
    
        let tmp = &ihdr.data;
    
        self.width = u_int32_to_int(&tmp[0..4]);
        if self.width <= 0 {
            return;
        }
    
        self.height = u_int32_to_int(&tmp[4..8]);
        if self.height <= 0 {
            return;
        }
    
        self.bit_depth = tmp[8] as i32;
        self.color_type = tmp[9] as i32;
    
        // Only compression method 0 is supported
        if tmp[10] as i32 != 0 {
            return;
        }
        self.compression = tmp[10] as i32;
    
        // Only filter method 0 is supported
        if tmp[11] as i32 != 0 {
            return;
        }
        self.filter = tmp[11] as i32;
    
        // Only interlace methods 0 and 1 are supported
        if tmp[12] as i32 != 0 {
            return;
        }
        self.interlace = tmp[12] as i32;
    }

    fn new() -> PNG {
        PNG { width: 0, height: 0, bit_depth: 0, color_type: 0, compression: 0, filter: 0, interlace: 0, chunks: Vec::new(), num_chunks: 0, dats: Vec::new() }
    }
}