use super::SceneElement;
use crate::data::{Hit, Ray, RTFloat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ElementList (Vec<SceneElement>);

impl ElementList {
    pub fn ray_hit(&self, ray: &Ray, t_min: RTFloat, t_max: RTFloat) -> Option<Hit> {
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