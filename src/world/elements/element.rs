use crate::data::{Point, Ray, Hit};

pub trait SceneElement: Send + Sync {
    fn get_position(&self) -> &Point;
    fn ray_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}