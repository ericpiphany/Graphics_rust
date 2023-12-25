use std::ops;

use crate::vec3;
#[derive(Copy,Clone,Default,Debug)]
pub struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
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
}
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
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            x: (self.x / rhs),
            y: (self.y / rhs),
            z: (self.z / rhs),
        }
    }
}
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
impl ops::Neg<> for Vec3{
    type Output = Vec3;
   fn neg(self) -> Self::Output {
       Vec3{
        x: -self.x,
        y: -self.y,
        z: -self.z
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
    fn d_euclidsq(&self) ->f64;
    fn cross(&self, rhs: Vec3) -> Vec3; //Cross product
    fn unit(&self) -> Vec3;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    //fn dsq_euclid(&self) -> f64;
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
    fn d_euclidsq(&self) ->f64 {
        self.x.powi(2)  + self.y.powi(2)+ self.z.powi(2)
    }
    fn unit(&self) -> Vec3 {
        self / self.d_euclid()
    }
    fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * rhs.z - self.z * rhs.y),
            y: (self.z * rhs.x - self.x * rhs.z),
            z: (self.x * rhs.y - self.y * rhs.x),
        }
    }
}
