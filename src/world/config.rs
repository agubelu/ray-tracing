use crate::data::Point;
use crate::img::ImgFormat;
use super::{camera, elements::ElementList};
use camera::Camera;

pub struct SceneConfig<'a> {
    pub img_width: usize,
    pub img_height: usize,
    pub camera: Camera,
    pub title: String,
    pub img_format: ImgFormat,
    pub elements: ElementList<'a>,
    pub samples: u32,
}

impl<'a> SceneConfig<'a> {
    pub fn new(img_width: usize, img_height: usize, vp_height: f32, focal_length: f32,
           camera_pos: Point, title: String, img_format: ImgFormat, elements: ElementList<'a>, samples: u32) -> Self {
            let aspect_ratio = img_width as f32 / img_height as f32;
            let camera = Camera::create(camera_pos, aspect_ratio, vp_height, focal_length);
            SceneConfig { img_width, img_height, camera, title, img_format, elements, samples }
    }
}