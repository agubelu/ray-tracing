use super::Material;
use crate::data::{Ray, Hit, Color, Vec3};

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn create(color: Color) -> Self {
        Lambertian { color }
    }

    pub fn boxed(color: Color) -> Box<Self> {
        Box::new(Self::create(color))
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let mut scatter_dir = hit.normal() + Vec3::random_unit();
        if scatter_dir.is_near_zero() {
            scatter_dir = *hit.normal();
        }
        let scattered = Ray::new(*hit.point(), scatter_dir);
        Some((self.color, scattered))
    }
}