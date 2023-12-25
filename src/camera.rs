use crate::colour;
use crate::colour::write_colour;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::VectorProperties;
use colour::Colour;
use interval::{Interval, UNIVERSE_INTERVAL};
#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    centre: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}
pub trait CameraProperties {
    fn initialize(&mut self) -> Self;
    fn render(&self, world: &dyn Hittable) -> ();
    fn ray_colour(r: &Ray, world: &dyn Hittable) -> colour::Colour;
}



impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            image_width,
            image_height: 0, // Set later in initialize
            centre: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            pixel00_loc: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            pixel_delta_u: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            pixel_delta_v: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        camera.initialize();
        camera
    }
}
impl CameraProperties for Camera {
    fn initialize(&mut self) -> Camera {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = self.image_height.max(1);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0 ;
        let viewport_width = viewport_height * self.aspect_ratio;

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let viewport_v = Vec3 {
            x: 0.0,
            y: -viewport_height,
            z: 0.0,
        };

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper-left pixel.
        let viewport_upper_left = self.centre
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            }
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.50 * (self.pixel_delta_u + self.pixel_delta_v);
        *self
    }
     fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j - 1);
            for i in 0..self.image_width {
                let pixel_centre = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_centre - self.centre;
                let r = Ray{ origin: self.centre, direction: ray_direction };

                let pixel_colour = Camera::ray_colour(&r, world);
                
                write_colour(&mut std::io::stdout(), pixel_colour);
            }
        }

        eprintln!("\rDone.                 ");
    }
    fn ray_colour(r: &Ray, world: &dyn Hittable) -> colour::Colour {
        let mut rec: Option<HitRecord> = Some(HitRecord::default());
        let ret_colour: Vec3;
        if world.hit(
            r,
            Interval {
                min: 0.0,
                max: UNIVERSE_INTERVAL.max,
            },
            &mut rec,
        ) {
            ret_colour = 0.5
                * (rec.unwrap().n
                    + Colour {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    });
            //eprintln!("{:?}",rec.unwrap());
        } else {
            let unit_direct = (r.direction).unit();
            let alpha = 0.5 * (unit_direct.y() + 1.0);
            ret_colour = (1.0 - alpha)
                * colour::Colour {
                    x: 1.0, //1.0 the new change is cool
                    y: 1.0,
                    z: 1.0,
                }
                + alpha
                    * colour::Colour {
                        x: 0.5,
                        y: 0.7,
                        z: 1.0,
                    };
        }
    
        ret_colour
        //Colour{x:1.0,y:1.0,z:1.0}
    }
}
