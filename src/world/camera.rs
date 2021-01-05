use crate::data::{Vec3, Point, Ray};
use config::CameraConfig;

use super::config;

pub struct Camera {
    position: Point,
    h: Vec3,
    v: Vec3,
    corner: Vec3,
}

impl Camera {
    pub fn create(look_from: Point, look_at: Point, vup: Vec3, aspect_ratio: f32, fov: f32) -> Self {
        let theta = fov.to_radians();
        let hor = (theta / 2.0).tan();
        let vp_height = 2.0 * hor;
        let vp_width = vp_height * aspect_ratio;

        let w = (look_from - look_at).unit();
        let u = (vup % w).unit();
        let v = w % u;

        let h = u * vp_width;
        let v = v * vp_height;
        let corner = look_from - h/2.0 - v/2.0 - w;
        Camera { h, v, corner, position: look_from }
    }

    pub fn from_config(config: CameraConfig) -> Self {
        Self::create(config.look_from, config.look_at, config.vup, config.aspect_ratio, config.fov)
    }

    pub fn create_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.position, self.corner + self.h * u + self.v * v - self.position)
    }
}

