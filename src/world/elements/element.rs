use crate::data::{Ray, Hit, Point, RTFloat};
use crate::world::materials::Material;
use super::sphere;
use serde::Deserialize;
use SceneElement::*;

/* 
   Having an enum is a bit less clean than having a Scene trait and some
   implementors, but it also makes deserializing easier and automatically
   extensible to new elements. It also helps avoiding indirections (Box<dyn Scene>)
   but this doesn't seem to have any significant effect in speed. 
*/
#[derive(Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SceneElement {
    Sphere { center: Point, radius: RTFloat, material: Material },
}

impl SceneElement {
    pub fn ray_hit(&self, ray: &Ray, t_min: RTFloat, t_max: RTFloat) -> Option<Hit> {
        match self {
            Sphere { center, radius, material } => sphere::sphere_hit(center, *radius, &material, ray, t_min, t_max),
        }
    }
}
