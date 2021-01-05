use super::super::materials::MaterialBox;
use super::SceneElement;
use crate::data::{Point, Ray, Hit};

pub struct Sphere<'a> {
    center: Point,
    radius: f32,
    material: MaterialBox<'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point, radius: f32, material: MaterialBox<'a>) -> Self {
        Sphere { center, radius, material }
    }

    pub fn boxed(center: Point, radius: f32, material: MaterialBox<'a>) -> Box<Self> {
        Box::new(Self::new(center, radius, material))
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl<'a> SceneElement for Sphere<'a> {
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
        let mut hit = Hit::new(point, normal, root, &self.material);
        hit.set_face_normal(&ray, &normal);
        return Some(hit);
    }

    fn get_position(&self) -> &Point {
        self.center()
    }
}
