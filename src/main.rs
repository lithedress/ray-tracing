use image::{ImageBuffer, RgbImage};

mod ray;
mod vector;

use ray::*;

mod solid_figures;

use solid_figures::*;

use num_traits::Float;
use rand::prelude::*;
use rand::Rng;
use std::rc::Rc;

fn main() {
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = 270;
    const SAMPLES_PER_IMAGE: u32 = 100;

    // World

    let world = HittableList {
        objects: vec![
            Rc::new(Sphere::new(Vector3::from_arr([0.0, 0.0, -1.0]), 0.5)),
            Rc::new(Sphere::new(Vector3::from_arr([0.0, -100.5, -1.0]), 100.0)),
        ],
    };

    // Camera

    let cam = Camera::new();

    // Render

    let img: RgbImage = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let mut color_f64 = Vector3::new();
        for s in 0..SAMPLES_PER_IMAGE {
            let u = (x as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
            let v = ((IMAGE_HEIGHT - y) as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
            let ray = cam.get_ray(u, v);
            color_f64 += ray.hit(&world);
        }
        color_f64.to_color(SAMPLES_PER_IMAGE)
    });
    img.save("output.png").expect("Can't save image!");
}

struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = 16.0 / 9.0 * viewport_height;
        let focal_length = 1.0;

        let origin = Vector3::new();
        let horizontal = Vector3::from_arr([viewport_width, 0.0, 0.0]);
        let vertical = Vector3::from_arr([0.0, viewport_height, 0.0]);
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vector3::from_arr([0.0, 0.0, focal_length]);
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray3<f64> {
        Ray3 {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        }
    }
}

fn ray_color_f64(ray: &Ray3<f64>, world: &dyn Hittable<f64>) -> [f64; 3] {
    if let Some(rec) = world.hit_by(ray, 0.0..f64::INFINITY) {
        let n = rec.normal;
        [n[0], n[1], n[2]]
    } else {
        let u = ray.direction.unitize();
        let t = 0.5 * (u[1] + 1.0);
        [
            ((1.0 - t) * 1.0 + t * 0.5),
            ((1.0 - t) * 1.0 + t * 0.7),
            ((1.0 - t) * 1.0 + t * 1.0),
        ]
    }
}

impl<F: Float> Ray3<F> {
    fn hit(&self, world: &dyn Hittable<F>) -> Vector3<F> {
        if let Some(rec) = world.hit_by(self, F::zero()..F::infinity()) {
            let n = rec.normal;
            n
        } else {
            let u = self.direction.unitize();
            let t = F::from(0.5).unwrap() * (u[1] + F::one());
            Vector3::from_arr([
                ((F::one() - t) + t * F::from(0.5).unwrap()),
                ((F::one() - t) + t * F::from(0.7).unwrap()),
                ((F::one() - t) + t * F::from(1.0).unwrap()),
            ])
        }
    }
}

fn f64s_to_color(f64s: [f64; 3], samples_per_pixel: u32) -> image::Rgb<u8> {
    image::Rgb(f64s.map(|f| ((f / samples_per_pixel as f64).clamp(0.0, 0.999) * 255.999) as u8))
}

impl Vector3<f64> {
    fn to_color(self, samples_per_pixel: u32) -> image::Rgb<u8> {
        image::Rgb(self.map(|f| ((f / samples_per_pixel as f64).clamp(0.0, 0.999) * 255.999) as u8))
    }
}

fn ray_color(ray: &Ray3<f64>, world: &dyn Hittable<f64>) -> image::Rgb<u8> {
    if let Some(rec) = world.hit_by(ray, 0.0..f64::INFINITY) {
        let n = rec.normal;
        image::Rgb([
            ((0.5 * (n[0] + 1.0)) * 255.999) as u8,
            ((0.5 * (n[1] + 1.0)) * 255.999) as u8,
            ((0.5 * (n[2] + 1.0)) * 255.999) as u8,
        ])
    } else {
        let u = ray.direction.unitize();
        let t = 0.5 * (u[1] + 1.0);
        image::Rgb([
            (((1.0 - t) * 1.0 + t * 0.5) * 255.999) as u8,
            (((1.0 - t) * 1.0 + t * 0.7) * 255.999) as u8,
            (((1.0 - t) * 1.0 + t * 1.0) * 255.999) as u8,
        ])
    }
}
