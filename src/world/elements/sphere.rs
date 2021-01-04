use super::SceneElement;
use crate::data::{Point, Color, Ray, Vec3, Hit};
use crate::vec3;
pub struct Sphere {
    center: Point,
    radius: f32,
    color: Color,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, color: Color) -> Self {
        Sphere { center, radius, color }
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}

impl SceneElement for Sphere {
    fn ray_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin() - self.center();
        let a = ray.direction().mag2();
        let b_h = oc * ray.direction();
        let c = oc.mag2() - self.radius() * self.radius();
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
        let normal = (point - self.center()) / self.radius();
        let mut hit = Hit::new(point, normal, root);
        hit.set_face_normal(&ray, &normal);
        return Some(hit);
    }

    fn get_position(&self) -> &Point {
        self.center()
    }
}
