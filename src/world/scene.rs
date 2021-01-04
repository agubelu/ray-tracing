use crate::vec3;
use crate::data::{Color, Point, Vec3, Ray};
use crate::img::{Image, create_image};
use super::{SceneConfig, Camera, elements::ElementList};

use rand::prelude::*;

pub struct Scene<'a> {
    img_width: usize,
    img_height: usize,
    camera: Camera,
    title: String,
    image: Box<dyn Image>,
    elements: ElementList<'a>,
    samples: u32,
}

impl<'a> Scene<'a> {
    pub fn from_config(config: SceneConfig<'a>) -> Self {
        Scene {
            img_width: config.img_width,
            img_height: config.img_height,
            camera: config.camera,
            title: config.title,
            image: create_image(config.img_width, config.img_height, &config.img_format),
            elements: config.elements,
            samples: config.samples,
        }
    }

    pub fn render(&mut self) {
        let max_w = (self.img_width - 1) as f32;
        let max_h = (self.img_height - 1) as f32;

        let background_grad_start: Color = vec3![1.0, 1.0, 1.0];
        let background_grad_end: Color = vec3![0.5, 0.7, 1.0];

        let mut rng = rand::thread_rng();

        for y in 0..self.img_height {
            for x in 0..self.img_width {
                let mut color = vec3![0.0, 0.0, 0.0];

                for _ in 0..self.samples {
                    let u = (x as f32 + rng.gen::<f32>()) / max_w;
                    let v = (y as f32 + rng.gen::<f32>()) / max_h;
                    let ray = self.camera.create_ray(u, v);

                    color += &self.get_ray_color(&ray, &background_grad_start, &background_grad_end);
                }
                
                color /= self.samples as f32;
                self.image.set_pixel(x, self.img_height-1 - y, &color);
            }
        }

        self.image.save(&self.title);
    }

    fn get_ray_color(&self, ray: &Ray, grad_start: &Color, grad_end: &Color) -> Color {
        if let Some(hit) = self.elements.ray_hit(ray, 0.0, 5.0) {
            return (hit.normal() + vec3![1.0, 1.0, 1.0]) * 0.5;
        }
        
        let unit_dir = ray.direction().unit();
        let t = (unit_dir.y() + 1.0) * 0.5;
        return grad_start * (1.0 - t) + grad_end * t;
    }
}