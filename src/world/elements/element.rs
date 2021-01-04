use crate::data::{Color, Point, Ray};

pub trait SceneElement {
    fn get_position(&self) -> &Point;
    fn ray_hit(&self, ray: &Ray) -> Option<Color>;
}