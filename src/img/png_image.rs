use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use super::Image;
use crate::data::Color;

extern crate png;

///////////////////////////////////////////////////////////////////////////////

const MAX_VAL: f32 = 255.0;

pub struct PNGImage {
    width: usize,
    height: usize,
    content: Vec<u8>,
}

impl PNGImage {
    pub fn new(width: usize, height: usize) -> Self {
        let content = vec![0; width * height * 3];
        PNGImage { width, height, content }
    }
}

impl Image for PNGImage {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        let pos = 3 * (y * self.width + x);
        self.content[pos] = (color[0] * MAX_VAL) as u8;
        self.content[pos + 1] = (color[1] * MAX_VAL) as u8;
        self.content[pos + 2] = (color[2] * MAX_VAL) as u8;
    }

    fn save(&self, filename: &str) {
        let out_path = format!("out/{}.png", filename);
        let path = Path::new(&out_path);
        let img_file = File::create(path).unwrap();

        let bw = BufWriter::new(img_file);
        let mut png_encoder = png::Encoder::new(bw, self.width() as u32, self.height() as u32);
        png_encoder.set_color(png::ColorType::RGB);
        png_encoder.set_depth(png::BitDepth::Eight);

        let mut img_writer = png_encoder.write_header().unwrap();
        img_writer.write_image_data(&self.content).unwrap();
    }
}