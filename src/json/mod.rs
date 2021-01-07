pub mod specs;
pub mod load;

pub use specs::{RenderSpec, SceneSpec, CameraSpec};
pub use load::load_scenes_from_file;