use crate::world::materials::Material;
use crate::data::{Point, Ray, Hit};

pub fn sphere_hit<'a>(center: &Point, radius: f32, material: &'a Material, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit<'a>> {
    let oc = ray.origin() - center;
    let a = ray.direction().mag2();
    let b_h = oc * ray.direction();
    let c = oc.mag2() - radius * radius;
    let disc = b_h*b_h - a*c;

    if disc < 0.0 {
        return None;
    }

    let disc_sqrt = disc.sqrt();
    let mut root = (-b_h - disc_sqrt) / a;
    if root < t_min || root > t_max {
        root = (-b_h + disc_sqrt) / a;
        if root < t_min || root > t_max {
            return None;
        }
    }

    let point = ray.at(root);
    let normal = (point - center) / radius;
    let mut hit = Hit::new(point, normal, root, material);
    hit.set_face_normal(&ray, &normal);
    
    return Some(hit);
}
