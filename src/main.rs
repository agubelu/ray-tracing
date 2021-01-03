mod containers;

use std::{char::MAX, fs::File, io::Write};
use std::io::BufWriter;
use containers::vec3::{Color, Vec3};


fn main() {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;
    const MAX_VAL: usize = 255;    
    /*
    let mut f = BufWriter::new(File::create("out/simple.ppm").expect("Unable to create file"));
    
    f.write(&format!("P3\n{} {}\n{}\n", WIDTH, HEIGHT, MAX_VAL).as_bytes());

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let r = MAX_VAL - i;
            let g = MAX_VAL - j;
            let b = 128;

            f.write(&format!("{} {} {}\n", r, g, b).as_bytes());
        }
    }*/

    
    println!("{:?}", vec3![1.0, 2.0, 3.0]);

}
