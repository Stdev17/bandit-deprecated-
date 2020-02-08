//extern crate image;
use std::time::{Instant};
//use image::GenericImageView;
//use std::{fs, mem};
use std::fs;
use std::io::{BufReader, BufRead, BufWriter, Write};

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    reader();
}

fn reader() {
    let now = Instant::now();
    /*
    let img = image::open("resources/kokkoro2.png").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
    println!("{}", now.elapsed().as_micros());

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();
    println!("{}", now.elapsed().as_micros());

    //own coded
    now = Instant::now();
    */
    let s = "dkdkdkdkdkdbcvckdkdkdk";

    let mut source = BufReader::new(fs::File::open("resources/kokkoro2.png").unwrap());
    //let mut b: [u8; 4] = unsafe { mem::MaybeUninit::uninit().assume_init() };
    let mut b: Vec<u8> = Vec::new();
    let mut m: usize;
    
    loop {
        m = source.read_until(b'-', &mut b).expect("");
        if m == 0 {
            break
        }
    }
    
    //println!("{:?}", f);
    println!("{}", b.len());
    println!("{}", now.elapsed().as_micros());


    let mut dest = BufWriter::new(fs::File::create("test.png").unwrap());
    for i in 0..b.len() {
        dest.write(&[b[i]]).unwrap();
    }
    dest.flush().unwrap();

    println!("{}", now.elapsed().as_micros());
}