use super::Material;
use crate::data::{Ray, Hit, Color, Vec3};

pub struct Metal {
    color: Color,
    fuzziness: f32,
}

impl Metal {
    pub fn create(color: Color, fuzziness: f32) -> Self {
        Metal { color, fuzziness }
    }

    pub fn boxed(color: Color , fuzziness: f32) -> Box<Self> {
        Box::new(Self::create(color, fuzziness))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&ray.direction().unit(), &hit.normal());
        let scattered = Ray::new(*hit.point(), reflected + Vec3::random_in_unit_sphere() * self.fuzziness);

        if scattered.direction() * hit.normal() > 0.0 {
            Some((self.color, scattered))
        } else {
            None
        }
    }
}