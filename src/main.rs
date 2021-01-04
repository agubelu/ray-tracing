mod data;
mod img;
mod world;

use std::fs::create_dir_all;

use data::Vec3;
use world::{SceneConfig, Scene, elements::{ElementList, Sphere}};
use img::ImgFormat;

fn main() {
    // Create the out/ folder if it doesn't exist
    create_dir_all("out").unwrap();

    let sphere = Box::new(Sphere::new(vec3![0.0, 0.0, -1.0], 0.5, vec3![1.0, 0.0, 0.0]));
    let floor = Box::new(Sphere::new(vec3![0.0, -1000.5, -1.0], 1000.0, vec3![1.0, 0.0, 0.0]));
    let mut elems = ElementList::new();
    elems.add_element(floor);
    elems.add_element(sphere);

    let config = SceneConfig::new(640, 360, 2.0, 1.0, vec3![0.0, 0.0, 0.0], "scene".into(), ImgFormat::PNG, elems, 100);
    let mut scene = Scene::from_config(config);
    scene.render();
}

