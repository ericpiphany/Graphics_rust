use crate::colour;
use crate::colour::{write_colour, Colour};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval, UNIVERSE_INTERVAL};
use crate::ray::Ray;
use crate::rtweekend::rand0_1;
use crate::vec3::{Vec3, VectorProperties};
use rayon::prelude::*;
#[derive(Debug, Clone, Copy, Default)]
 pub(crate)  struct  Camera {
    pub(crate) aspect_ratio: f64,
    pub(crate) image_width: i32,
    pub(crate) image_height: i32,
    pub(crate) max_depth: i32,
    pub(crate) vfov: f64,
    pub(crate) samples_per_pixel: i32,
    pub(crate)centre: Vec3,
    pub(crate)pixel00_loc: Vec3,
    pub(crate)pixel_delta_u: Vec3, //Delta vector
    pub(crate)pixel_delta_v: Vec3,
    pub(crate) lookfrom: Vec3,
    pub(crate) lookat: Vec3,
    pub(crate) vup: Vec3,
    pub(crate)u: Vec3, //Camera basis vectors
    pub(crate)v: Vec3,
    pub(crate)w:  Vec3,
}
pub trait CameraProperties {
    fn initialize(&mut self) -> Self;
    fn render(&self, world: &dyn Hittable) -> ();
    fn ray_colour(r: &Ray, depth: i32, world: &dyn Hittable) -> colour::Colour;
    fn get_ray(&self, i: f64, j: f64) -> Ray;
    fn sample_square(&self) -> Vec3;
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            image_width,
            image_height: 0, // Set later in initialize
            samples_per_pixel,
            max_depth: 10, //default ray depth
            vfov: 90.0,    //Default vertical view angle.
            lookfrom: Vec3 {
                x: 0.0,
                y: 3.0,
                z: 3.0, // this takes us a little bit farther away (z = 3.0)
            },          // and lifts us a little (y = 3.0)
            lookat: Vec3 {
                x: 0.5,
                y: 0.0,
                z: -1.5, // just for a fun "not looking straight at the object center"
            },           // as the image center seems to be built around (0, 0, -1)
                         // with these choices we will be looking at a downwards angle
            vup: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            centre: Vec3::default(),
            pixel00_loc: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),

        };

        camera.initialize();
        camera
    }
}
impl CameraProperties for Camera {
    fn initialize(&mut self) -> Camera {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = self.image_height.max(1);
        self.centre = self.lookfrom;
        // Determine viewport dimensions.
        let focal_length = (self.lookfrom - self.lookat).d_euclid();
        eprintln!("{}",focal_length);
        let theta = self.vfov.to_radians();
        eprintln!("{}",theta);
        let h = (theta / 2.0).tan();
        eprintln!("{}",h);
        let viewport_height = 2.0 * h * focal_length ;
        eprintln!("{}",viewport_height);
        let viewport_width = viewport_height * self.aspect_ratio;
        eprintln!("{}",viewport_width);
        //Calculate orthonormal basis for cam coord frame
        self.w = (self.lookfrom - self.lookat).unit();
        self.u = (self.vup.cross(&self.w)).unit();
        self.v = (self.w).cross(&self.u);
        //eprintln!(" w {:?}\n u {:?}\n v{:?}\n lookat {:?}\n lookfrom {:?}",self.w,self.u,self.v,self.lookat,self.lookfrom);
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u =  viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper-left pixel.
        let viewport_upper_left = self.centre - (focal_length * self.w) -  (viewport_u + viewport_v)/2.0;
        self.pixel00_loc = viewport_upper_left +  (self.pixel_delta_u + self.pixel_delta_v)/2.0;
        //eprintln!(" w {:?}\n u {:?}\n v{:?}\n lookat {:?}\n lookfrom {:?}",self.w,self.u,self.v,self.lookat,self.lookfrom);
        //eprintln!(" view_u {:?}\n view_v {:?}\n pix_d_u {:?}\n pix_d_v {:?}\n v_u_l {:?}",viewport_u,viewport_v,self.pixel_delta_u,self.pixel_delta_v,viewport_upper_left);
        //eprintln!("pix_loc {:?}", self.pixel00_loc);
        *self
    }

    // ...
    fn render(&self, world: &dyn Hittable) {
        // Create a parallel iterator over the rows and collect the results
        let pixel_rows: Vec<Vec<Colour>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                (0..self.image_width)
                    .into_par_iter()
                    .map(|i| {
                        let pixel_centre = self.pixel00_loc
                            + (i as f64 * self.pixel_delta_u)
                            + (j as f64 * self.pixel_delta_v);
                        let _ray_direction = pixel_centre - self.centre;
                        let mut pixel_colour: Vec3 = Default::default();
                        for _ in 0..self.samples_per_pixel {
                            let ray_r = self.get_ray(i.into(), j.into());
                            pixel_colour += Self::ray_colour(&ray_r, self.max_depth, world);
                        }

                        pixel_colour
                    })
                    .collect()
            })
            .collect();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        // Flatten the collected results and print them sequentially
        for row in pixel_rows {
            for pixel_colour in row {
                write_colour(&mut std::io::stdout(), pixel_colour, self.samples_per_pixel);
            }
        }

        eprintln!("\rDone.");
    }

    fn ray_colour(r: &Ray, depth: i32, world: &dyn Hittable) -> colour::Colour {
        let mut rec: Option<HitRecord> = Some(HitRecord::default());
        let ret_colour: Vec3;
        if depth <= 0 {
            return Colour::default();
        }
        if world.hit(
            r,
            Interval {
                min: 0.001,
                max: UNIVERSE_INTERVAL.max,
            },
            &mut rec,
        ) {
            let mut scattered = Ray::default();
            let mut attenuation = Colour::default();
            let inner_record = rec.as_ref().unwrap();
            ret_colour = match inner_record.mat_type.scatter(
                r,
                &inner_record,
                &mut attenuation,
                &mut scattered,
            ) {
                //true => attenuation * Self::ray_colour(&scattered, depth - 1, world),
                true => attenuation.element_wise_multiply(&Self::ray_colour(
                    &scattered,
                    depth - 1,
                    world,
                )),
                false => Colour::default(),
            };
            //eprintln!("Hit: {:?}", inner_record);
        } else {
            let unit_direct = r.direction.unit();
            let alpha = 0.5 * (unit_direct.y() + 1.0);
            ret_colour = (1.0 - alpha)
                * colour::Colour {
                    x: 1.0,
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
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        //Gets a randomly sampled camera ray for pixel at current location
        let pixel_centre = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
        let pixel_sample = pixel_centre + self.sample_square();
        let ray_origin = self.centre;
        let ray_direction = pixel_sample - ray_origin;
        Ray {
            origin: ray_origin,
            direction: ray_direction,
        }
    }

    fn sample_square(&self) -> Vec3 {
        let px = -0.5 + rand0_1();
        let py = -0.5 + rand0_1();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }
}
