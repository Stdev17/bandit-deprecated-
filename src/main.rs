//extern crate image;

mod img;
use img::parse::readpng;
mod algo;

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    //reader();
    img::simple::main();
}

fn reader() {
    
    /*
    let img = image::open("resources/kokkoro2.png").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
    println!("{}", now.elapsed().as_micros());

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();

    //own coded
    now = Instant::now();
    

    // PNG header
    for i in 0..8 {
        dest.write(&[b[i]]).unwrap();
    }
    // IHDR Chunk
    for i in 8..33 {
        dest.write(&[b[i]]).unwrap();
    }

    for i in (33..b.len()-4).step_by(4) {
        let g: u8 = (gray(b[i], b[i+1], b[i+2])*255.0) as u8;
        dest.write(&[g, g, g, b[i+3]]).unwrap();
    }
    for i in b.len()-4..b.len() {
        dest.write(&[b[i]]).unwrap();
    }

    dest.flush().unwrap();

    println!("{}", now.elapsed().as_micros());
    */
    readpng();
}