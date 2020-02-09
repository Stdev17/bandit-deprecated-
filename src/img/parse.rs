
use std::time::{Instant};
//use image::GenericImageView;
//use std::{fs, mem};
use std::{fs, str};
use std::io::{BufReader, BufRead, Read, BufWriter};//, Write};
use std::string::String;
//mod algo;
//use algo::grayscale::gray;
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

// uInt32ToInt converts a 4 byte big-endian buffer to int.
fn u_int32_to_int(buf: &Vec<u8>) -> i32 {
    if buf.len() == 0 || buf.len() > 4 {
        return 0;
    }
    let mut res: i32 = 0;
    for i in 0..4 {
        res += 2i32.pow(i*8)*(buf[i as usize] as i32);
    }
    return res;
}

pub fn readpng() {
    let now = Instant::now();
    let mut source = BufReader::new(fs::File::open("resources/kokkoro2.png").unwrap());
    //let mut b: [u8; 4] = unsafe { mem::MaybeUninit::uninit().assume_init() };
    let mut b: Vec<u8> = Vec::new();
    //println!("{}", type_of(&source));
    let mut m: usize;
    
    loop {
        m = source.read_until(b' ', &mut b).expect("");
        if m == 0 {
            break
        }
    }
    let mut c = Chunk::new();
    let p = Chunk::populate(&mut c, &mut source);
    
    //println!("{:?}", f);
    println!("{}", b.len());
    println!("{}", now.elapsed().as_micros());

    let mut dest = BufWriter::new(fs::File::create("test.png").unwrap());
}

impl Chunk {
    // Populate will read bytes from the reader and populate a chunk.
    fn populate(&mut self, s: &mut BufReader<std::fs::File>) {
        
        // Four byte buffer.
        let mut buf: Vec<u8> = Vec::with_capacity(4);

        // Read first four bytes == chunk length.
        for i in 0..4 {
            s.read_exact(&mut buf).unwrap();
        }
        println!("{:?}", &buf);
        // Convert bytes to int.
        // c.length = int(binary.BigEndian.Uint32(buf))
        self.length = u_int32_to_int(&buf);

        let buf2: Vec<u8> = vec![0; 4];
        // Read second four bytes == chunk type.
        //
        //self.c_type = str::from_utf8(&buf2);

        // Read chunk data.
        let mut tmp: Vec<u8> = Vec::with_capacity(self.length as usize);
        //
        self.data = tmp;

        // Read CRC32 hash
        //
        // We don't really care about checking the hash.
        self.crc32 = buf;
    }

    fn new() -> Chunk {
        Chunk { length: 0, c_type: "".to_string(), data: Vec::new(), crc32: Vec::new() }
    }
}