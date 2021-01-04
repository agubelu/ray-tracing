use super::{Point, Vec3, Ray};
#[derive(Copy, Clone)]
pub struct Hit {
    point: Point,
    normal: Vec3,
    t: f32,
    front: bool,
}

impl Hit {
    pub fn new(point: Point, normal: Vec3, t: f32) -> Self {
        Hit { point, normal, t, front: true }
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

    pub fn set_face_normal(&mut self, ray: &Ray, out_norm: &Vec3) {
        self.front = ray.direction() * out_norm < 0.0;
        self.normal = if self.front {*out_norm} else {out_norm * -1.0};
    }
}