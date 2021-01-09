use crate::data::{Ray, Hit, Color, Vec3, RTFloat};

pub fn metal_scatter(color: &Color, fuzziness: RTFloat, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
    let reflected = Vec3::reflect(&ray.direction().unit(), &hit.normal());
    let scattered = Ray::new(*hit.point(), reflected + Vec3::random_in_unit_sphere() * fuzziness);

    if scattered.direction() * hit.normal() > 0.0 {
        Some((*color, scattered))
    } else {
        None
    }
}
