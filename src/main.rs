use std::rc::Rc;
use image::{ImageBuffer, RgbImage};

mod ray;
mod vector;
mod solid_figures;

use solid_figures::*;
use rand::prelude::*;

fn main() {
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = 270;
    const SAMPLES_PER_IMAGE: u32 = 100;
    const MAX_DEPTH: i32 = 50;

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
        for _ in 0..SAMPLES_PER_IMAGE {
            let u = (x as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
            let v = ((IMAGE_HEIGHT - y) as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
            let ray = cam.get_ray(u, v);
            color_f64 += ray.hit(&world, MAX_DEPTH);
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

impl Ray3<f64> {
    fn hit(&self, world: &dyn Hittable<f64>, depth: i32) -> Vector3<f64> {
        if depth <= 0 {
            return Vector3::new();
        }
        if let Some(rec) = world.hit_by(self, 0.001..f64::INFINITY) {
            let target = rec.p + rec.normal + Vector3::new_random_in_unit().unitize();
            Ray3 {origin: rec.p, direction: target - rec.p}.hit(world, depth - 1) * 0.5
        } else {
            let u = self.direction.unitize();
            let t = 0.5 * (u.0[1] + 1.0);
            Vector3::from_arr([
                ((1.0 - t) + t * 0.5),
                ((1.0 - t) + t * 0.7),
                ((1.0 - t) + t * 1.0),
            ])
        }
    }
}

impl Vector3<f64> {
    fn to_color(self, samples_per_pixel: u32) -> image::Rgb<u8> {
        image::Rgb(self.map(|f| ((f / samples_per_pixel as f64).sqrt().clamp(0.0, 0.999) * 255.999) as u8))
    }

    fn new_random_in_unit() -> Self {
        let mut ans = Self::new();
        loop {
            for f in &mut ans.0 {
                *f = random();
            }
            if ans.norm_pow2() < 1.0 {
                return ans;
            }
        }
    }
}
