use crate::vec3;
use crate::data::{Color, Vec3, Ray};
use crate::img::{ImgFormat, save_image};
use super::{SceneConfig, Camera, elements::ElementList};

use rand::random;
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;

pub struct Scene<'a> {
    img_width: usize,
    img_height: usize,
    img_format: ImgFormat,
    camera: Camera,
    title: String,
    elements: ElementList<'a>,
    samples: u32,
}

impl<'a> Scene<'a> {
    pub fn from_config(config: SceneConfig<'a>) -> Self {
        Scene {
            img_width: config.img_width,
            img_height: config.img_height,
            img_format: config.img_format,
            camera: Camera::from_config(config.camera),
            title: config.title,
            elements: config.elements,
            samples: config.samples,
        }
    }

    pub fn render(&mut self) {
        const MAX_BOUNCES: u32 = 50;

        let max_w = (self.img_width - 1) as f32;
        let max_h = (self.img_height - 1) as f32;

        let background_grad_start: Color = vec3![1.0, 1.0, 1.0];
        let background_grad_end: Color = vec3![0.5, 0.7, 1.0];
        let gamma_correct = 1.0 / self.samples as f32;

        let mut pixels = vec![0; self.img_width * self.img_height * 3];
        let slices = pixels.par_chunks_mut(self.img_width * 3);

        (0..self.img_height).into_par_iter().rev().zip(slices)
                            .progress_count(self.img_height as u64)
                            .for_each(|(y, chunk)| {
            for x in 0..self.img_width {
                let mut color = vec3![0.0, 0.0, 0.0];

                for _ in 0..self.samples {
                    let u = (x as f32 + random::<f32>()) / max_w;
                    let v = (y as f32 + random::<f32>()) / max_h;
                    let ray = self.camera.create_ray(u, v);

                    color += &self.get_ray_color(&ray, MAX_BOUNCES, &background_grad_start, &background_grad_end);
                }

                let pos = x * 3;
                chunk[pos] = ((color.x() * gamma_correct).sqrt() * 255.0) as u8;
                chunk[pos + 1] = ((color.y() * gamma_correct).sqrt() * 255.0) as u8;
                chunk[pos + 2] = ((color.z() * gamma_correct).sqrt() * 255.0) as u8;
            }
        });

        save_image(self.img_width, self.img_height, &pixels, &self.img_format, &self.title);
    }

    fn get_ray_color(&self, ray: &Ray, bounces_left: u32, grad_start: &Color, grad_end: &Color) -> Color {
        if bounces_left == 0 {
            return vec3![0.0, 0.0, 0.0];
        }

        if let Some(hit) = self.elements.ray_hit(ray, 0.0001, f32::MAX) {
            if let Some((color, scattered)) = hit.material().scatter(ray, &hit) {
                let ray_color = self.get_ray_color(&scattered, bounces_left - 1, grad_start, grad_end);
                return color.elem_prod(&ray_color);
            } else {
                return vec3![0.0, 0.0, 0.0];
            }
        }
        
        let unit_dir = ray.direction().unit();
        let t = (unit_dir.y() + 1.0) * 0.5;
        return grad_start * (1.0 - t) + grad_end * t;
    }
}