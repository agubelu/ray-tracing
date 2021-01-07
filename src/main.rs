mod data;
mod img;
mod json;
mod world;

use json::load_scenes_from_file;
use std::{env, process};
use std::fs::create_dir_all;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide the paths to one or more JSON files containing the scenes to render.");
        process::exit(1);
    }

    // Create the out/ folder if it doesn't exist
    create_dir_all("out").unwrap();

    // Render all scenes from the provided files
    for file in &args[1..] {
        println!("--- Rendering scenes from {} ---", file);
        for scene in load_scenes_from_file(file) {
            println!("\t · {}", scene.get_title());
            scene.render();
        }
    }
}
