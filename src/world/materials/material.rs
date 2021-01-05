use crate::data::{Ray, Hit, Color};

pub type MaterialBox<'a> = Box<dyn Material + 'a>;
pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
}