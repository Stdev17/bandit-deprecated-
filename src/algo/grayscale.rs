
#[inline]
pub fn gray(x: u8, y: u8, z: u8) -> f32 {
    let clin: f32 = (0.2126*x as f32 + 0.7152*y as f32 + 0.0722*z as f32)/255.0;
    let csrgb: f32;
    if clin > 0.0031308 {
        csrgb = 1.055 * clin.powf(1.0/2.4) - 0.055;
    } else {
        csrgb = 12.92 * clin;
    }
    return csrgb;
}