#[inline]
fn gray(x: f32, y: f32, z: f32) -> f32 {
    let clin: f32 = (0.2126*x + 0.7152*y + 0.0722*z)/255.0;
    let csrgb: f32;
    if clin > 0.0031308 {
        csrgb = 1.055 * clin.powf(1.0/2.4) - 0.055;
    } else {
        csrgb = 12.92 * clin;
    }
    return csrgb;
}

#[inline]
fn clamp_u8 (x: f32) -> u8 {
    if x > 255.0 {
        return 255u8;
    } else if x < 0.0 {
        return 0u8;
    } else {
        return x as u8;
    }
}

pub fn set(mut b: Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut calc: u8 = 0;
    println!("{}", b.len());
    for i in 0..((b.len() as i32 - 1) / 4 + 1) {
        calc = clamp_u8(gray(b[(i*4) as usize] as f32, b[(i*4+1) as usize] as f32, b[(i*4+2) as usize] as f32));
        /*
        res.push(calc);
        res.push(calc);
        res.push(calc);
        res.push(b[i*4+3]);
        */
        b[(i*4) as usize] = calc;
        b[(i*4+1) as usize] = calc;
        b[(i*4+2) as usize] = calc;
    }
    return res;
}