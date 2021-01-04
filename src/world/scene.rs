use crate::vec3;
use crate::data::{Color, Point, Vec3, Ray};
use crate::img::{Image, create_image};
use super::{SceneConfig, elements::SceneElement};

pub struct Scene<'a> {
    img_width: usize,
    img_height: usize,
    vp_width: f32,
    vp_height: f32,
    focal_length: f32,
    camera_pos: Point,
    title: String,
    image: Box<dyn Image>,
    elements: Vec<Box<dyn SceneElement + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn from_config(config: SceneConfig<'a>) -> Self {
        let aspect_ratio = config.img_width as f32 / config.img_height as f32;
        let vp_width = config.vp_height as f32 * aspect_ratio;
        
        Scene {
            vp_width,
            img_width: config.img_width,
            img_height: config.img_height,
            vp_height: config.vp_height as f32,
            focal_length: config.focal_length,
            camera_pos: config.camera_pos,
            title: config.title,
            image: create_image(config.img_width, config.img_height, &config.img_format),
            elements: config.elements,
        }
    }

    pub fn render(&mut self) {
        let max_w = (self.img_width - 1) as f32;
        let max_h = (self.img_height - 1) as f32;

        let background_grad_start: Color = vec3![1.0, 1.0, 1.0];
        let background_grad_end: Color = vec3![0.5, 0.7, 1.0];

        let origin = &self.camera_pos;
        let hor = vec3![self.vp_width, 0.0, 0.0];
        let ver = vec3![0.0, self.vp_height, 0.0];
        let lower_left_corner = origin - hor/2.0 - &ver/2.0 - vec3![0.0, 0.0, self.focal_length];

        for y in 0..self.img_height {
            for x in 0..self.img_width {
                let u = x as f32 / max_w;
                let v = y as f32 / max_h;
                let ray_dir = lower_left_corner + hor * u + ver * v - origin;
                let ray = Ray::new(&origin, &ray_dir);

                let color = self.get_ray_color(&ray, &background_grad_start, &background_grad_end);
                self.image.set_pixel(x, self.img_height-1 - y, &color);
            }
        }

        self.image.save(&self.title);
    }

    fn get_ray_color(&self, ray: &Ray, grad_start: &Color, grad_end: &Color) -> Color {
        for elem in &self.elements {
            if let Some(color) = elem.ray_hit(ray) {
                return color;
            }
        }

        let unit_dir = ray.direction().unit();
        let t = (unit_dir.y() + 1.0) * 0.5;
        return grad_start * (1.0 - t) + grad_end * t;
    }
}