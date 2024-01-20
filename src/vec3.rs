use crate::rtweekend;
use rtweekend::{rand0_1, rand_range};
use std::{f64, iter::repeat_with, ops};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}
impl Vec3 {
    #[inline]
    pub fn random() -> Vec3 {
        Vec3 {
            x: rand0_1(),
            y: rand0_1(),
            z: rand0_1(),
        }
    }
    #[inline]
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: rand_range(min, max),
            y: rand_range(min, max),
            z: rand_range(min, max),
        }
    }
    
    pub fn random_in_unit_sphere() -> Vec3 {
        repeat_with(|| Vec3::random_range(-1.0, 1.0))
            .filter(|p| p.d_euclidsq() < 1.0)
            .next()
            .unwrap()
    }
    #[inline]
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }
    #[inline]
    pub fn random_on_hemisphere(norm: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        match on_unit_sphere * norm > 0.0 {
            true => on_unit_sphere,
            false => -on_unit_sphere,
        }
    }
    #[inline]
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - ((*v * *n) * *n) * 2.0 //v - 2*dot(v,n)*n;
    }
    #[inline]
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv*n).min(1.0);
        let r_out_perp = etai_over_etat * ( *uv + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.d_euclidsq()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }
    #[inline]
    pub fn element_wise_multiply(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
    
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
        }
    }
}
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
            z: (self.z - rhs.z),
        }
    }
}
impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: (self.x * rhs),
            y: (self.y * rhs),
            z: (self.z * rhs),
        }
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: (self * rhs.x),
            y: (self * rhs.y),
            z: (self * rhs.z),
        }
    }
} //Scalar operation

impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Vec3) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) //Dot operator
    }
}
impl ops::Mul<&Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) //Dot operator
    }
}
//impl ops::DivAssign<f64> for Vec3 {
//    fn div_assign(&mut self, rhs: f64) {
//        *self = Vec3 {
//            x: (self.x / rhs),
//            y: (self.y / rhs),
//            z: (self.z / rhs),
//        }
//    }
//}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: (self.x / rhs),
            y: (self.y / rhs),
            z: (self.z / rhs),
        }
    }
}
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: (self.x / rhs),
            y: (self.y / rhs),
            z: (self.z / rhs),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub trait VectorProperties {
    fn d_euclid(&self) -> f64;
    fn d_euclidsq(&self) -> f64;
    fn cross(&self, rhs: &Vec3) -> Vec3; //Cross product
    fn unit(&self) -> Vec3;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn near_zero(&self) -> bool;
}

impl VectorProperties for Vec3 {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
    fn d_euclid(&self) -> f64 {
        f64::sqrt(self.d_euclidsq())
    }
    fn d_euclidsq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    fn unit(&self) -> Vec3 {
        self / self.d_euclid()
    }
    fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * rhs.z - self.z * rhs.y),
            y: (self.z * rhs.x - self.x * rhs.z),
            z: (self.x * rhs.y - self.y * rhs.x),
        }
    }
    fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}
