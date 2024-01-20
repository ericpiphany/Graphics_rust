#[path = "vec3.rs"]
mod vec3;
pub use crate::{colour::vec3::Vec3, vec3::Vec3 as Colour};
use crate::{interval::Interval, vec3::VectorProperties};
use std::io::Write;
#[inline]
fn linear_to_gamma(lin: f64) -> f64 {
    lin.sqrt()
}
pub fn write_colour(mut handler: impl Write, pixel_colour: Colour, samples_per_pixel: i32) -> () {
    let scale = 1.0 / samples_per_pixel as f64;
    let intensity: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };
    let rgb: (f64, f64, f64) = (
        intensity.clamp(linear_to_gamma(pixel_colour.x() * scale)),
        intensity.clamp(linear_to_gamma(pixel_colour.y() * scale)),
        intensity.clamp(linear_to_gamma(pixel_colour.z() * scale)),
    );

    let ir = (256.0 * rgb.0 as f64) as i32;
    let ig = (256.0 * rgb.1 as f64) as i32;
    let ib = (256.0 * rgb.2 as f64) as i32;
    writeln!(handler, "{} {} {}", ir.abs(), ig.abs(), ib.abs()).expect("Failed to write to standard device passed in handler");
}
