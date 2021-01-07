use super::SceneElement;
use crate::data::{Hit, Ray};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ElementList (Vec<SceneElement>);

impl ElementList {
    pub fn ray_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut res = None;
        let mut t_closest = t_max;

        for element in &self.0 {
            if let Some(hit) = element.ray_hit(ray, t_min, t_closest) {
                t_closest = hit.t();
                res = Some(hit);
            }
        }

        return res;
    }
}