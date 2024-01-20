use crate::colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::rand0_1;
use crate::vec3::Vec3;
use crate::vec3::VectorProperties;
use colour::Colour;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Colour },
    Default { albedo: Colour },
    Metal { albedo: Colour, fuzz: f64 }, // Add more material types as needed
    Dielectric { idx_refract: f64 },
}
impl Default for Material {
    fn default() -> Self {
        Material::Default {
            albedo: Colour::default(),
        }
    }
}

impl Material {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let rtheta = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        rtheta + (1.0 - rtheta) * (1.0 - cosine).powi(5)
        //Reflectivity Schlick approximation
    }
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Default { albedo } => {
                let mut scatter_direction = rec.n + Vec3::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.n;
                }
                *scattered = Ray {
                    origin: rec.p,
                    direction: scatter_direction,
                };
                *attenuation = *albedo;
                true
            } // Default has the same implementation as Lambertian
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.n + Vec3::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.n;
                }
                *scattered = Ray {
                    origin: rec.p,
                    direction: scatter_direction,
                };
                *attenuation = *albedo;
                true
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(&r_in.direction.unit(), &rec.n);
                *scattered = Ray {
                    origin: rec.p,
                    direction: reflected + *fuzz * Vec3::random_unit_vector(),
                };
                *attenuation = *albedo;
                true
            }
            Material::Dielectric { idx_refract } => {
                *attenuation = Colour {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                };
                let refraction_ratio = match rec.front_face {
                    true => 1.0 / idx_refract,
                    false => *idx_refract,
                };
                let unit_direction = r_in.direction.unit();
                let cos_theta = (-unit_direction * rec.n).min(1.0);
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let refracted = match cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rand0_1() {
                    true => Vec3::reflect(&unit_direction, &rec.n),
                    false => Vec3::refract(&unit_direction, &rec.n, refraction_ratio),
                };

                //let refracted = Vec3::refract(&unit_direction, &rec.n,refraction_ratio);
                *scattered = Ray {
                    origin: rec.p,
                    direction: refracted,
                };

                true
            }
        }
    }
}
