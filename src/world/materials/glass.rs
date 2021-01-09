use rand::random;

use crate::data::{Ray, Hit, Color, Vec3, RTFloat};

pub fn glass_scatter(color: &Color, refraction_index: RTFloat, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
    let ref_ratio = if hit.is_front() {1.0 / refraction_index} else {refraction_index};
    let dir = ray.direction().unit();

    let mut cos_theta  = (dir * -1.0) * hit.normal();
    if cos_theta  > 1.0 {
        cos_theta  = 1.0;
    }

    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let cant_refract = ref_ratio * sin_theta > 1.0;
    let direction = if cant_refract || reflectance(cos_theta, ref_ratio) > random() {
        Vec3::reflect(&dir, hit.normal())
    } else {
        Vec3::refract(&dir, hit.normal(), ref_ratio)
    };

    let scattered = Ray::new(*hit.point(), direction);
    Some((*color, scattered))
}

fn reflectance(cos: RTFloat, ref_idx: RTFloat) -> RTFloat {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}