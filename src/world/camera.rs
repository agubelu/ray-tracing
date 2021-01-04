use crate::vec3;
use crate::data::{Vec3, Point, Ray};

pub struct Camera {
    position: Point,
    h: Vec3,
    v: Vec3,
    corner: Vec3,
}

impl Camera {
    pub fn create(position: Point, aspect_ratio: f32, vp_height: f32, focal: f32) -> Self {
        let vp_width = vp_height * aspect_ratio;
        let h = vec3![vp_width, 0.0, 0.0];
        let v = vec3![0.0, vp_height, 0.0];
        let corner = position - h/2.0 - v/2.0 - vec3![0.0, 0.0, focal];
        Camera { position, h, v, corner }
    }

    pub fn create_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.position, self.corner + self.h * u + self.v * v)
    }
}

