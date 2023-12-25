use crate::interval;
use crate::{
    ray::{Ray, RayProperties},
    vec3::{Vec3, VectorProperties},
};
use interval::{Interval};

use std::vec::Vec;
use std::{
    sync::{Arc},
};
#[derive(Clone, Copy, Default, Debug)]
pub struct HitRecord {
    pub(crate) p: Vec3,
    pub(crate) n: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

#[derive(Default)]
pub struct Sphere {
    pub(crate) centre: Vec3,
    pub(crate) radius: f64,
}
pub struct HittableList {
    pub(crate) objects: Vec<Arc<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}
impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Option<HitRecord>) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.iter() {
            if object.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                rec,
            ) {
                hit_anything = true;
                closest_so_far = rec.as_ref().map_or(ray_t.max, |temp_rec| temp_rec.t);
            }
        }

        hit_anything
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Option<HitRecord>) -> bool {
        let mut temp_rec: Option<HitRecord> = Some(HitRecord::default());
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.unwrap().t;
                *rec = Some(temp_rec.unwrap().clone()); // Wrap temp_rec in Some and assign to rec
            }
        }

        hit_anything
    }
}

trait HitRecordProperties {
    fn set_face_normal(r: &Ray, outward_normal: &Vec3, rec: &mut Option<HitRecord>) -> ();
}
impl HitRecordProperties for HitRecord {
    fn set_face_normal(r: &Ray, outward_normal: &Vec3, rec: &mut Option<HitRecord>) -> () {
        let front_face = r.direction() * outward_normal < 0.0;
        let mut norm = outward_normal.clone();
        if front_face {
        } else {
            norm = -norm;
        };
        rec.as_mut().map(|hit_record| {
            hit_record.n = norm.clone();
        });
    }
}
pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Option<HitRecord>) -> bool;
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Option<HitRecord>) -> bool {
        let oc = r.origin - self.centre;
        let a = r.direction.d_euclidsq();
        let half_b = oc * r.direction;
        let c = oc.d_euclidsq() - self.radius * self.radius;
        let discrim = half_b * half_b - a * c;

        if discrim <= 0.0 {
            return false;
        }

        let root = discrim.sqrt();
        let temp = {
            let t1 = (-half_b - root) / a;
            let t2 = (-half_b + root) / a;
            if ray_t.surrounds(t1) {
                t1
            } else if ray_t.surrounds(t2) {
                t2
            } else {
                return false;
            }
        };

        let hit_point = r.at(temp);
        let outward_norm = (hit_point - self.centre) / self.radius;
        let front_face = r.direction * outward_norm < 0.0;

        *rec = Some(HitRecord {
            p: hit_point,
            n: if front_face { outward_norm } else { -outward_norm },
            t: temp,
            front_face,
        });

        true
    }
}
