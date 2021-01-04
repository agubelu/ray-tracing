use crate::data::Point;
use crate::img::ImgFormat;
use elements::SceneElement;

use super::elements;

pub struct SceneConfig<'a> {
    pub img_width: usize,
    pub img_height: usize,
    pub vp_height: usize,
    pub focal_length: f32,
    pub camera_pos: Point,
    pub title: String,
    pub img_format: ImgFormat,
    pub elements: Vec<Box<dyn SceneElement + 'a>>,
}

impl<'a> SceneConfig<'a> {
    pub fn new(img_width: usize, img_height: usize, vp_height: usize, focal_length: f32,
           camera_pos: Point, title: String, img_format: ImgFormat, elements: Vec<Box<dyn SceneElement + 'a>>) -> Self {
            SceneConfig { img_width, img_height, vp_height, focal_length, camera_pos, title, img_format, elements }
    }
}