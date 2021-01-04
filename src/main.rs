mod data;
mod img;
mod world;

use std::fs::create_dir_all;

use data::Vec3;
use world::{SceneConfig, Scene, elements::{SceneElement, Sphere}};
use img::ImgFormat;

fn main() {
    // Create the out/ folder if it doesn't exist
    create_dir_all("out").unwrap();

    let sphere = Box::new(Sphere::new(vec3![0.0, 0.0, -1.0], 0.5, vec3![1.0, 0.1, 0.1]));
    let elems: Vec<Box<dyn SceneElement>> = vec![sphere];

    let config = SceneConfig::new(400, 225, 2, 1.0, vec3![0.0, 0.0, 0.0], "scene".into(), ImgFormat::PNG, elems);
    let mut scene = Scene::from_config(config);
    scene.render();
}

