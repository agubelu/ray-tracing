pub mod vec3;
pub mod tests;
pub mod ray;
pub mod hit;

pub use vec3::{Vec3, VecElem, Color, Point};
pub use ray::Ray;
pub use hit::Hit;