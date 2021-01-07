use crate::data::{Vec3, Point, Ray};
use crate::json::CameraSpec;

pub struct Camera {
    position: Point,
    hor: Vec3,
    ver: Vec3,
    corner: Vec3,
    lens_rad: f32,
}

impl Camera {
    pub fn create(look_from: Point, look_at: Point, vup: Vec3, aspect_ratio: f32, fov: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let vp_height = 2.0 * h;
        let vp_width = vp_height * aspect_ratio;

        let w = (look_from - look_at).unit();
        let u = (vup % w).unit();
        let v = w % u;

        let hor = u * vp_width * focus_dist;
        let ver = v * vp_height * focus_dist;
        let corner = look_from - hor/2.0 - ver/2.0 - w*focus_dist;
        let lens_rad = aperture / 2.0;
        Camera { hor, ver, corner, lens_rad, position: look_from }
    }

    pub fn from_spec(spec: CameraSpec, aspect_ratio: f32) -> Self {
        Self::create(spec.position, spec.looking_at, spec.up_vec, aspect_ratio, spec.fov, spec.aperture, spec.focus_distance)
    }

    pub fn create_ray(&self, u: f32, v: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_rad;
        let offset = rd.x() * u + rd.y() + v;
        Ray::new(
            self.position + offset,
            self.corner + self.hor * u + self.ver * v - self.position - offset
        )
    }
}
