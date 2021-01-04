use super::SceneElement;
use crate::data::{Hit, Ray};

pub type BoxedElement<'a> = Box<dyn SceneElement + 'a>;

pub struct ElementList<'a> {
    elements: Vec<BoxedElement<'a>>
}

impl<'a> ElementList<'a> {
    pub fn new() -> Self {
        ElementList { elements: Vec::new() }
    }

    pub fn add_element(&mut self, elem: BoxedElement<'a>) {
        self.elements.push(elem);
    }

    pub fn ray_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut res = None;
        let mut t_closest = t_max;

        for element in &self.elements {
            if let Some(hit) = element.ray_hit(ray, t_min, t_closest) {
                t_closest = hit.t();
                res = Some(hit);
            }
        }

        return res;
    }
}