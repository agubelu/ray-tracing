mod data;
mod img;
mod world;

use std::fs::create_dir_all;

use data::Vec3;
use world::{
    config::CameraConfig,
    SceneConfig, 
    Scene, 
    elements::{
        ElementList, 
        Sphere,
    },
    materials::{
        Metal, Lambertian, Glass
    }
};
use img::ImgFormat;

fn main() {
    // Create the out/ folder if it doesn't exist
    create_dir_all("out").unwrap();

    
    let left_sphere = Sphere::boxed(
        vec3![0.0, -0.2, -0.4], 
        0.2, 
        Glass::boxed(vec3![0.8, 0.6, 0.6], 1.5)
    );

    let left_sphere2 = Sphere::boxed(
        vec3![0.0, -0.2, -0.4], 
        -0.19, 
        Glass::boxed(vec3![0.8, 0.6, 0.6], 1.5)
    );

    let center_sphere = Sphere::boxed(
        vec3![0.0, 0.0, -1.0], 
        0.5, 
        Lambertian::boxed(vec3![0.1, 0.2, 0.5])
    );

    let right_sphere = Sphere::boxed(
        vec3![1.0, 0.0,-1.0], 
        0.5, 
        Metal::boxed(vec3![0.9, 0.9, 0.9], 0.05)
    );

    let floor_sphere = Sphere::boxed(
        vec3![0.0, -100.5,-1.0], 
        100.0, 
        Lambertian::boxed(vec3![0.8, 0.8, 0.0])
    );

    let mut elems = ElementList::new();
    elems.push(floor_sphere);
    elems.push(right_sphere);
    elems.push(left_sphere);
    elems.push(left_sphere2);
    elems.push(center_sphere);

    let camera = CameraConfig {
        look_from: vec3![-1.5, 1.5, 1.0],
        look_at: vec3![0.0, 0.0, -1.0],
        vup: vec3![0.0, 1.0, 0.0],
        fov: 45.0,
        aspect_ratio: 16.0 / 9.0,
        aperture: 0.1,
        focus_dist: (vec3![-1.5, 1.5, 1.0] - vec3![0.0, 0.0, -1.0]).mag()
    };

    let config = SceneConfig::new(1280, 720, camera, "scene".into(), ImgFormat::PNG, elems, 25);
    let mut scene = Scene::from_config(config);
    scene.render();
}

