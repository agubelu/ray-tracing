use super::SceneElement;
use crate::data::{Point, Color, Ray};
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
    fn ray_hit(&self, ray: &Ray) -> Option<Color> {
        let oc = ray.origin() - self.center();
        let a = ray.direction() * ray.direction();
        let b = oc * ray.direction() * 2.0;
        let c = oc * oc - self.radius() * self.radius();
        let disc = b*b - 4.0*a*c;

        if disc > 0.0 {
            Some(*self.color())
        } else {
            None
        }
    }

    fn get_position(&self) -> &Point {
        self.center()
    }
}
