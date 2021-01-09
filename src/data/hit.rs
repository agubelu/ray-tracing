use super::{Point, Vec3, Ray, RTFloat};
use crate::world::materials::Material;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    point: Point,
    normal: Vec3,
    t: RTFloat,
    front: bool,
    material: &'a Material,
}

impl<'a> Hit<'a> {
    pub fn new(point: Point, normal: Vec3, t: RTFloat, material: &'a Material) -> Self {
        Hit { point, normal, t, material, front: true }
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn t(&self) -> RTFloat {
        self.t
    }

    pub fn material(&self) -> &Material {
        self.material
    }

    pub fn is_front(&self) -> bool {
        self.front
    }

    pub fn set_face_normal(&mut self, ray: &Ray, out_norm: &Vec3) {
        self.front = ray.direction() * out_norm < 0.0;
        self.normal = if self.front {*out_norm} else {out_norm * -1.0};
    }
}