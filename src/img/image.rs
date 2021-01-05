use crate::data::Color;
use super::{ImgFormat, encoders};

const MAX_VAL: f32 = 255.0;

pub struct ImageBuf {
    width: usize,
    height: usize,
    content: Vec<u8>,
}

impl ImageBuf {
    pub fn new(width: usize, height: usize) -> Self {
        ImageBuf {
            width, height, content: vec![0; width * height * 3]
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: &Color) {
        let pos = 3 * (y * self.width() + x);
        self.content[pos] = (pixel[0] * MAX_VAL) as u8;
        self.content[pos + 1] = (pixel[1] * MAX_VAL) as u8;
        self.content[pos + 2] = (pixel[2] * MAX_VAL) as u8;
    }

    pub fn save(&self, filename: &str, format: &ImgFormat) {
        let (ext, encoder) = match format {
            ImgFormat::PNG => ("png", encoders::encode_png)
        };

        let out_path = format!("out/{}.{}", filename, ext);
        encoder(self.width(), self.height(), &self.content, &out_path);
    }
}