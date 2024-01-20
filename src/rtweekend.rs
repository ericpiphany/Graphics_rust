use rand::prelude::*;
//const PI: f64 = std::f64::consts::PI;
//const INFINITY: f64 = f64::INFINITY;

//#[inline(always)]
//pub fn degrees_to_radians(deg: f64) -> f64 {
//    deg * PI / 180.0
//}
#[inline]
pub fn rand0_1() -> f64{
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1
    y
}
#[inline]
pub fn rand_range(min:f64,max:f64) -> f64{
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(min..max); // generates a float between 0 and 1
    y
}
