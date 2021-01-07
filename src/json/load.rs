use crate::world::Scene;
use super::RenderSpec;
use std::fs::read_to_string;

pub fn load_scenes_from_file(path: &str) -> Vec<Scene> {
    let json_str = read_to_string(path).expect("Error reading file!");
    let spec: RenderSpec = serde_json::from_str(&json_str).expect("Error deserializing data!");
    spec.scenes.into_iter().map(Scene::from_spec).collect()
}
