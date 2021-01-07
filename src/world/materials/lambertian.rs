use crate::data::{Ray, Hit, Color, Vec3};

pub fn lambertian_scatter(color: &Color, _: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
    let mut scatter_dir = hit.normal() + Vec3::random_unit();
    if scatter_dir.is_near_zero() {
        scatter_dir = *hit.normal();
    }
    let scattered = Ray::new(*hit.point(), scatter_dir);
    Some((*color, scattered))
}
