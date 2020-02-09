

#[inline]
fn sinc(x: f32) -> f32 {
    if x == 0.0 {
        1.0
    } else {
        let a: f32 = x * std::f32::consts::PI;
        a.sin() / a
    }
}

#[inline]
pub fn lanczos(x: f32) -> f32 {
    let taps: f32 = 3.0;
    if x.abs() < taps {
        sinc(x) * sinc(x / taps)
    } else {
        0.0
    }
}