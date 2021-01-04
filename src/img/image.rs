use crate::data::Color;
use super::{ImgFormat, PNGImage};

pub trait Image {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set_pixel(&mut self, x: usize, y: usize, color: &Color);
    fn save(&self, filename: &str);
}

pub fn create_image(width: usize, height: usize, ext: &ImgFormat) -> Box<dyn Image> {
    Box::new(match ext {
        ImgFormat::PNG => PNGImage::new(width, height),
    })
}   