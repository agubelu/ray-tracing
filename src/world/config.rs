use crate::data::{Point, Vec3};
use crate::img::ImgFormat;
use super::elements::ElementList;

pub struct SceneConfig<'a> {
    pub img_width: usize,
    pub img_height: usize,
    pub camera: CameraConfig,
    pub title: String,
    pub img_format: ImgFormat,
    pub elements: ElementList<'a>,
    pub samples: u32,
}

pub struct CameraConfig {
    pub look_from: Point, 
    pub look_at: Point, 
    pub vup: Vec3, 
    pub aspect_ratio: f32, 
    pub fov: f32,
}

impl<'a> SceneConfig<'a> {
    pub fn new(img_width: usize, img_height: usize, camera: CameraConfig, title: String, img_format: ImgFormat, elements: ElementList<'a>, samples: u32) -> Self {
            SceneConfig { img_width, img_height, camera, title, img_format, elements, samples }
    }
}