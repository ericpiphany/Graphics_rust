#[path = "vec3.rs"]
mod vec3;
use crate::vec3::VectorProperties;
pub use crate::{colour::vec3::Vec3, vec3::Vec3 as Colour};
use std::io::{Write};
pub fn write_colour(mut handler:  impl Write, pixel_colour: Colour) -> () {
    let ir = (255.999 * pixel_colour.x() as f64) as i32;
    let ig = (255.999 * pixel_colour.y() as f64) as i32;
    let ib = (255.999 * pixel_colour.z() as f64) as i32;
    writeln!(handler, "{} {} {}", ir.abs(), ig.abs(), ib.abs());
}
