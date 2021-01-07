use overload::overload;
use serde::Deserialize;
use std::ops;

pub type Color = Vec3;
pub type Point = Vec3;
pub type VecElem = f32;

const NEAR_ZERO: f32 = 1e-8;

#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub struct Vec3 ([VecElem; 3]);

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => { Vec3::from($x, $y, $z) };
    () => { Vec3::new() };
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 { 0: [0.0; 3] }
    }

    pub fn from(e1: VecElem, e2: VecElem, e3: VecElem) -> Self {
        Vec3 { 0: [e1, e2, e3] }
    }

    pub fn random() -> Self {
        Vec3 { 0: [rand::random(), rand::random(), rand::random()] }
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        Self::random() * (max - min) + min
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random_range(-1.0, 1.0);
            if v.mag2() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let v = Self::from(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0, 0.0);
            if v.mag2() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        v - n * 2.0 * v.dot_prod(&n)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, e: f32) -> Self {
        let mut cos_theta = (uv * -1.0) * n;
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }

        let perp = (uv + (n * cos_theta)) * e;
        let par = n * -(1.0 - perp.mag2()).abs().sqrt();
        perp + par
    }

    pub fn x(&self) -> VecElem {
        self[0]
    }

    pub fn y(&self) -> VecElem {
        self[1]
    }

    pub fn z(&self) -> VecElem {
        self[2]
    }

    pub fn mag(&self) -> VecElem {
        self.mag2().sqrt()
    }

    pub fn mag2(&self) -> VecElem {
        self[0]*self[0] + self[1]*self[1] + self[2]*self[2]
    }

    pub fn unit(&self) -> Self {
        self / self.mag()
    }

    pub fn dot_prod(&self, other: &Self) -> VecElem {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn elem_prod(&self, other: &Self) -> Vec3 {
        Self::from(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }

    pub fn cross_prod(&self, other: &Self) -> Self {
        Self::from(self[1] * other[2] - self[2] * other[1],
                   self[2] * other[0] - self[0] * other[2], 
                   self[0] * other[1] - self[1] * other[0])
    }

    pub fn is_near_zero(&self) -> bool {
        self[0] < NEAR_ZERO && self[1] < NEAR_ZERO && self[2] < NEAR_ZERO
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = VecElem;

    fn index(&self, ind: usize) -> &VecElem {
        &self.0[ind]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, ind: usize) -> &mut VecElem {
        &mut self.0[ind]
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
    }
}

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &Vec3) {
        self[0] -= other[0];
        self[1] -= other[1];
        self[2] -= other[2];
    }
}

impl ops::MulAssign<VecElem> for Vec3 {
    fn mul_assign(&mut self, mult: VecElem) {
        self[0] *= mult;
        self[1] *= mult;
        self[2] *= mult;
    }
}

impl ops::DivAssign<VecElem> for Vec3 {
    fn div_assign(&mut self, div: VecElem) {
        self[0] /= div;
        self[1] /= div;
        self[2] /= div;
    }
}

overload!((a: ?Vec3) + (b: ?Vec3) -> Vec3 { Vec3 { 0: [a[0] + b[0], a[1] + b[1], a[2] + b[2]] } });
overload!((a: ?Vec3) + (b: VecElem) -> Vec3 { Vec3 { 0: [a[0] + b, a[1] + b, a[2] + b] } });
overload!((a: ?Vec3) - (b: ?Vec3) -> Vec3 { Vec3 { 0: [a[0] - b[0], a[1] - b[1], a[2] - b[2]] } });
overload!((a: ?Vec3) - (b: VecElem) -> Vec3 { Vec3 { 0: [a[0] - b, a[1] - b, a[2] - b] } });
overload!((a: ?Vec3) * (b: VecElem) -> Vec3 { Vec3 { 0: [a[0] * b, a[1] * b, a[2] * b] } });
overload!((a: ?Vec3) / (b: VecElem) -> Vec3 { Vec3 { 0: [a[0] / b, a[1] / b, a[2] / b] } });
overload!((a: ?Vec3) * (b: ?Vec3) -> VecElem { a.dot_prod(&b) });
overload!((a: ?Vec3) % (b: ?Vec3) -> Vec3 { a.cross_prod(&b) });