
use std::time::{Instant};
//use image::GenericImageView;
//use std::{fs, mem};
use std::{fs, str};
use std::io::{BufReader, Read, BufWriter, Write};
use std::string::String;
use crate::algo::grayscale::set;
use crc::crc32;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
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
    let mut source = BufReader::with_capacity(2048, fs::File::open("resources/kokkoro2.png").unwrap());
    let mut dest = BufWriter::with_capacity(1024, fs::File::create("test.png").unwrap());
    println!("{}", now.elapsed().as_micros());
    //let mut b: [u8; 4] = unsafe { mem::MaybeUninit::uninit().assume_init() };
    let mut header: Vec<u8> = vec![0; 8];
    println!("{}", now.elapsed().as_micros());
    source.read_exact(&mut header).unwrap();
    println!("{}", now.elapsed().as_micros());
    dest.write_all(&header).unwrap();
    
    /*
    let mut m: usize;
    
    loop {
        m = source.read_until(b' ', &mut b).expect("");
        if m == 0 {
            break
        }
    }
    */
    println!("{}", now.elapsed().as_micros());
    
    let mut ihdr = Chunk::new();
    Chunk::populate(&mut ihdr, &mut source, &mut dest);
    
    let mut png = PNG::new();
    PNG::parse_ihdr(&mut png, &mut ihdr);
    PNG::add(&mut png, ihdr);

    println!("{}", now.elapsed().as_micros());

    loop {
        let mut ch = Chunk::new();
        Chunk::populate(&mut ch, &mut source, &mut dest);
        let chk = &ch.c_type.clone();
        //println!("{}", &chk);
        PNG::add(&mut png, ch);
        if chk == "IEND" {
            break;
        }
    }

    println!("{}", now.elapsed().as_micros());

    let mut m: Vec<u8> = Vec::new();
    let mut cr: Vec<u8> = Vec::new();
    for i in png.chunks.iter_mut() {
        if i.c_type == "IDAT" {
            m.append(&mut i.data);
        }
        if i.c_type == "IEND" {
            
            let mut idat: Vec<u8> = vec![73, 68, 65, 84];
            idat.append(&mut m.clone());
            let mut c: u32 = crc32::checksum_ieee(&idat);
            //println!("{:X}", c);
            //assert_eq!(c, 0x21C8871F);
            
            let mut v: Vec<u8> = vec![0; 4];
            for j in 0..4 {
                v[j] = (c%256) as u8;
                c /= 256;
            }
            i.crc32 = v.clone();
            cr = v.clone();
            v = vec![0; 4];
            let mut l: u32 = m.len() as u32;
            i.length = l as i32;
            for j in 0..4 {
                v[j] = (l%256) as u8;
                l /= 256;
            }
            for i in 0..v.len() {
                dest.write(&[v[3-i]]).unwrap();
            }
            idat = vec![73, 68, 65, 84];
            for i in 0..idat.len() {
                dest.write(&[idat[i]]).unwrap();
            }
            
        }
    }
    println!("{}", now.elapsed().as_micros());

    for i in 0..m.len() {
        dest.write(&[m[i]]).unwrap();
    }

    for i in 0..cr.len() {
        dest.write(&[cr[3-i]]).unwrap();
    }
    let no: Vec<u8> = vec![0, 0, 0, 0];
    for i in 0..no.len() {
        dest.write(&[no[i]]).unwrap();
    }
    let iend: Vec<u8> = vec![73, 69, 78, 68];
    for i in 0..iend.len() {
        dest.write(&[iend[i]]).unwrap();
    }
    let mut c: u32 = crc32::checksum_ieee(&iend);
    let mut v: Vec<u8> = vec![0; 4];
    for j in 0..4 {
        v[j] = (c%256) as u8;
        c /= 256;
    }
    for i in 0..v.len() {
        dest.write(&[v[3-i]]).unwrap();
    }

    //println!("{:?}", f);
    //println!("{}", &png.chunks.len());
    
    println!("{}", now.elapsed().as_micros());

    dest.flush().unwrap();
    /*
    let mut chk = BufWriter::new(fs::File::create("testcrc.txt").unwrap());
    for i in 0..png.chunks[5].data.len() {
        chk.write(&[png.chunks[5].data[i]]).unwrap();
    }
    chk.flush().unwrap();
    */
}


impl Chunk {
    // Populate will read bytes from the reader and populate a chunk.
    fn populate(&mut self, s: &mut BufReader<std::fs::File>, dest: &mut BufWriter<std::fs::File>) {
        
        // Four byte buffer.
        let mut buf: Vec<u8> = vec![0; 4];

        // Read first four bytes == chunk length.
        s.read_exact(&mut buf).unwrap();

        //println!("{:?}", &buf);
        // Convert bytes to int.
        // c.length = int(binary.BigEndian.Uint32(buf))
        dest.write_all(&buf).unwrap();
        self.length = u_int32_to_int(&buf[0..4]);
        // Read second four bytes == chunk type.
        s.read_exact(&mut buf).unwrap();
        
        self.c_type = str::from_utf8(&buf).unwrap().to_string();
        if self.c_type != "IEND" && self.c_type != "IDAT" {
            dest.write_all(&buf).unwrap();
        }

        // Read chunk data.
        
        let mut tmp: Vec<u8> = vec![0; self.length as usize];
        s.read_exact(&mut tmp).unwrap();
        self.data = tmp;
        if self.c_type != "IEND" && self.c_type != "IDAT" {
            dest.write_all(&self.data).unwrap();
        }

        // Read CRC32 hash
        let mut buf3: Vec<u8> = vec![0; 4];
        // Read second four bytes == chunk type.
        s.read_exact(&mut buf3).unwrap();
        self.crc32 = buf3;
        // We don't really care about checking the hash.
        if self.c_type != "IEND" && self.c_type != "IDAT" {
            dest.write_all(&self.crc32).unwrap();
        }
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
        PNG { width: 0, height: 0, bit_depth: 0, color_type: 0, compression: 0, filter: 0, interlace: 0, chunks: Vec::new(), num_chunks: 0 }
    }
}