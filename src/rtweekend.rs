const PI: f64 = std::f64::consts::PI;
const INFINITY: f64 = f64::INFINITY;

#[inline(always)]
pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}
