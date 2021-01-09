use crate::data::{Ray, Hit, Color, RTFloat};
use crate::json::specs::deserialize_color;
use super::{glass, lambertian, metal};
use serde::Deserialize;
use Material::*;

/* 
   Having an enum is a bit less clean than having a Material trait and some
   implementors, but it also makes deserializing easier and automatically
   extensible to new materials. It also helps avoiding indirections (Box<dyn Material>)
   but this doesn't seem to have any significant effect in speed. 
*/
#[derive(Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Material {
    Lambertian { 
        #[serde(deserialize_with = "deserialize_color")]
        color: Color 
    },
    Glass { 
        #[serde(deserialize_with = "deserialize_color")]
        color: Color, 
        refraction_index: RTFloat
    },
    Metal { 
        #[serde(deserialize_with = "deserialize_color")]
        color: Color, 
        fuzziness: RTFloat 
    }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        match self {
            Lambertian { color } => lambertian::lambertian_scatter(color, ray, hit),
            Glass { color, refraction_index } => glass::glass_scatter(color, *refraction_index, ray, hit),
            Metal { color, fuzziness } => metal::metal_scatter(color, *fuzziness, ray, hit),
        }
    }
}
