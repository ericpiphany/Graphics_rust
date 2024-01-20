use std::{f64::consts::PI, sync::Arc};

use hittable::{HittableList, Sphere};
use material::Material;

mod colour;
use crate::vec3::Vec3;
mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod vec3;
use camera::{Camera, CameraProperties};

fn main() {
    let material_ground = Material::Lambertian {
        albedo: Vec3 {
            x: (0.8),
            y: (0.8),
            z: (8.0),
        },
    };
    let material_centre = Material::Lambertian {
        albedo: Vec3 {
            x: (0.1),
            y: (0.2),
            z: (0.5),
        },
    };
    let material_left = Material::Dielectric { idx_refract: (1.5) };
    let materia_right = Material::Metal {
        albedo: Vec3 {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        fuzz: (0.0),
    };

    //let material_dia = Material::Dielectric { idx_refract: (1.5) };
    let R = (PI / 4.0).cos();
    let camera = Camera::new(1.0, 800, 10);
    let mut world = HittableList::new();
    world.push(Arc::new(Sphere {
        centre: Vec3 {
            x: 0.0,
            y: -100.0,
            z: -1.0,
        },
        radius: 100.0,
        mat_type: material_ground,
    })); //Ground sphere lambertian
    world.push(Arc::new(Sphere {
        centre: Vec3 {
            x: 0.0,
            y: -0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat_type: material_centre,
    })); //Centre sphere lambertian
    world.push(Arc::new(Sphere {
        centre: Vec3 {
            x: -1.0,
            y: -0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat_type: material_left,
    })); //left1 sphere dielectric
    world.push(Arc::new(Sphere {
        centre: Vec3 {
            x: -1.0,
            y: -0.0,
            z: -1.0,
        },
        radius: -0.4,
        mat_type: material_left,
    })); //left2 sphere dielectric
    world.push(Arc::new(Sphere {
        centre: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat_type: material_left,
    })); //right sphere metal

    let cam_set = Camera {
        lookfrom: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        lookat: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vup:Vec3{ x: 0.0, y: 1.0, z: 0.0},
        ..camera
    };
    cam_set.render(&world);
}
