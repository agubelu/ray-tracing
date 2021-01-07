use crate::data::{Vec3, Color};
use crate::world::elements::ElementList;
use crate::img::ImgFormat;
use serde::{Deserialize, Deserializer};

pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s: ColorSpec = Deserialize::deserialize(deserializer)?;
    let r = s.0[0] as f32 / 255.0;
    let g = s.0[1] as f32 / 255.0;
    let b = s.0[2] as f32 / 255.0;
    Ok(Vec3::from(r, g, b))
}

#[derive(Deserialize)]
pub struct ColorSpec(pub [u8; 3]);

#[derive(Deserialize)]
pub struct CameraSpec {
    pub position: Vec3,
    pub looking_at: Vec3,
    pub up_vec: Vec3,
    pub fov: f32,
    pub aperture: f32,
    pub focus_distance: f32
}

#[derive(Deserialize)]
pub struct SceneSpec {
    pub img_width: usize,
    pub img_height: usize,
    pub title: String,
    pub out_format: ImgFormat,
    pub max_bounces: u32,
    pub antialias_amount: u32,
    pub camera: CameraSpec,
    pub elements: ElementList,
    #[serde(deserialize_with = "deserialize_color")]
    pub background_top_color: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub background_bottom_color: Color,
}

#[derive(Deserialize)]
pub struct RenderSpec {
    pub scenes: Vec<SceneSpec>,
}
