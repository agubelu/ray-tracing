use rand::random;

use super::Material;
use crate::data::{Ray, Hit, Color, Vec3};

pub struct Glass {
    color: Color,
    refraction_index: f32,
}

impl Glass {
    pub fn create(color: Color, refraction_index: f32) -> Self {
        Glass { color, refraction_index }
    }

    pub fn boxed(color: Color, refraction_index: f32) -> Box<Self> {
        Box::new(Self::create(color, refraction_index))
    }

    fn reflectance(&self, cos: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
    }
}

impl Material for Glass {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let ref_ratio = if hit.is_front() {1.0 / self.refraction_index} else {self.refraction_index};
        let dir = ray.direction().unit();

        let mut cos_theta  = (dir * -1.0) * hit.normal();
        if cos_theta  > 1.0 {
            cos_theta  = 1.0;
        }

        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cant_refract = ref_ratio * sin_theta > 1.0;
        let direction = if cant_refract || self.reflectance(cos_theta, ref_ratio) > random() {
            Vec3::reflect(&dir, hit.normal())
        } else {
            Vec3::refract(&dir, hit.normal(), ref_ratio)
        };

        let scattered = Ray::new(*hit.point(), direction);
        Some((self.color, scattered))
    }
}