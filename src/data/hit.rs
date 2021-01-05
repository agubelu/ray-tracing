use super::{Point, Vec3, Ray};
use crate::world::materials::MaterialBox;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    point: Point,
    normal: Vec3,
    t: f32,
    front: bool,
    material: &'a MaterialBox<'a>,
}

impl<'a> Hit<'a> {
    pub fn new(point: Point, normal: Vec3, t: f32, material: &'a MaterialBox<'a>) -> Self {
        Hit { point, normal, t, material, front: true }
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn material(&self) -> &MaterialBox {
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