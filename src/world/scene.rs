use crate::vec3;
use crate::data::{Color, Vec3, Ray, RTFloat};
use crate::img::{ImgFormat, save_image};
use crate::json::SceneSpec;
use super::{Camera, elements::ElementList};

use rand::random;
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;

pub struct Scene {
    img_width: usize,
    img_height: usize,
    img_format: ImgFormat,
    camera: Camera,
    title: String,
    elements: ElementList,
    antialias_amount: u32,
    max_bounces: u32,
    grad_start: Color,
    grad_end: Color,
}

impl Scene {
    pub fn from_spec(spec: SceneSpec) -> Self {
        let aspect_ratio = spec.img_width as RTFloat / spec.img_height as RTFloat;
        let cam = Camera::from_spec(spec.camera, aspect_ratio);

        Scene {
            img_width: spec.img_width,
            img_height: spec.img_height,
            img_format: spec.out_format,
            camera: cam,
            title: spec.title,
            elements: spec.elements,
            antialias_amount: spec.antialias_amount,
            max_bounces: spec.max_bounces,
            grad_start: spec.background_top_color,
            grad_end: spec.background_bottom_color
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn render(&self) {
        let max_w = (self.img_width - 1) as RTFloat;
        let max_h = (self.img_height - 1) as RTFloat;

        let gamma_correct = 1.0 / self.antialias_amount as RTFloat;

        let mut pixels = vec![0; self.img_width * self.img_height * 3];
        let slices = pixels.par_chunks_mut(self.img_width * 3);

        (0..self.img_height).into_par_iter().rev().zip(slices)
                            .progress_count(self.img_height as u64)
                            .for_each(|(y, chunk)| {
            for x in 0..self.img_width {
                let mut color = vec3![0.0, 0.0, 0.0];

                for _ in 0..self.antialias_amount {
                    let u = (x as RTFloat + random::<RTFloat>()) / max_w;
                    let v = (y as RTFloat + random::<RTFloat>()) / max_h;
                    let ray = self.camera.create_ray(u, v);

                    color += &self.get_ray_color(&ray, self.max_bounces);
                }

                let pos = x * 3;
                chunk[pos] = ((color.x() * gamma_correct).sqrt() * 255.0) as u8;
                chunk[pos + 1] = ((color.y() * gamma_correct).sqrt() * 255.0) as u8;
                chunk[pos + 2] = ((color.z() * gamma_correct).sqrt() * 255.0) as u8;
            }
        });

        save_image(self.img_width, self.img_height, &pixels, &self.img_format, &self.title);
    }

    fn get_ray_color(&self, ray: &Ray, bounces_left: u32) -> Color {
        if bounces_left == 0 {
            return vec3![0.0, 0.0, 0.0];
        }

        if let Some(hit) = self.elements.ray_hit(ray, 0.0001, RTFloat::MAX) {
            if let Some((color, scattered)) = hit.material().scatter(ray, &hit) {
                let ray_color = self.get_ray_color(&scattered, bounces_left - 1);
                return color.elem_prod(&ray_color);
            } else {
                return vec3![0.0, 0.0, 0.0];
            }
        }
        
        let unit_dir = ray.direction().unit();
        let t = (unit_dir.y() + 1.0) * 0.5;
        return self.grad_start * (1.0 - t) + self.grad_end * t;
    }
}