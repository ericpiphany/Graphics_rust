use std::borrow::Borrow;
use std::f64::INFINITY;
use std::io::{self, Write};
use std::ops;
use std::process::Output;
use std::sync::Arc;

use colour::Colour;
use hittable::{HitRecord, Hittable, HittableList, Sphere};
use ray::{Ray, RayProperties};
use vec3::VectorProperties;
mod colour;
use crate::vec3::Vec3;
mod camera;
mod hittable;
mod interval;
mod ray;
mod rtweekend;
mod vec3;
use camera::{Camera, CameraProperties};
use interval::{Interval, EMPTY_INTERVAL, UNIVERSE_INTERVAL};

fn main() {
    let camera = Camera::new(16.0 / 9.0, 1080);
    let mut world = HittableList::new(); 
    world.push(Arc::new(Sphere {
        centre: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));
    world.push(Arc::new(Sphere {
        centre: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));
    //let world: Arc<dyn Hittable> = Arc::new(hittable_list);
    camera.render(&world);
}
