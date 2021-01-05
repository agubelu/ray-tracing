pub mod material;
pub mod lambertian;
pub mod metal;
pub mod glass;

pub use material::{Material, MaterialBox};
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use glass::Glass;