#[path = "vec3.rs"]
mod vec3;

use crate::vec3::Vec3;


pub struct Ray {
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
}

pub trait RayProperties {
    fn origin(&self) -> Vec3;
    fn direction(&self) -> Vec3;
    fn at(&self, t: f64) -> Vec3;
}

impl RayProperties for Ray {
    fn origin(&self) -> Vec3 {
        self.origin
    }

    fn direction(&self) -> Vec3 {
        self.direction
    }

    fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
