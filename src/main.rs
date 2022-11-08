use image::{ImageBuffer, RgbImage};
use std::sync::{Arc, Mutex};
use std::thread;

mod double_horizon;
mod hittable;
mod material;
mod ray;
mod solid_figures;
mod vector;

use double_horizon::{Camera, Lambertian, Metal};
use hittable::HittableList;
use rand::prelude::*;
use solid_figures::*;

fn main() {
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = 270;
    const SAMPLES_PER_IMAGE: u32 = 96;
    const THREADS: u32 = 8;
    const MAX_DEPTH: i32 = 48;

    // World

    let material_ground = Arc::new(Lambertian {
        albedo: Color3::from_arr([0.8, 0.8, 0.0]),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Color3::from_arr([0.7, 0.3, 0.3]),
    });
    let material_left = Arc::new(Metal {
        albedo: Color3::from_arr([0.8, 0.8, 0.8]),
        fuzz: 0.3,
    });
    let material_right = Arc::new(Metal {
        albedo: Color3::from_arr([0.8, 0.6, 0.2]),
        fuzz: 1.0,
    });

    let world = Arc::new(HittableList {
        objects: vec![
            Arc::new(Sphere::new(
                Position3::from_arr([0.0, -100.5, -1.0]),
                100.0,
                material_ground,
            )),
            Arc::new(Sphere::new(
                Position3::from_arr([0.0, 0.0, -1.0]),
                0.5,
                material_center,
            )),
            Arc::new(Sphere::new(
                Position3::from_arr([-1.0, 0.0, -1.0]),
                0.5,
                material_left,
            )),
            Arc::new(Sphere::new(
                Position3::from_arr([1.0, 0.0, -1.0]),
                0.5,
                material_right,
            )),
        ],
    });

    // Camera

    let cam = Arc::new(Camera::new());

    // Render

    let img: RgbImage = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let color_f64 = Arc::new(Mutex::new(Color3::new()));
        for _ in 0..SAMPLES_PER_IMAGE / THREADS {
            let mut handles = vec![];
            for _ in 0..THREADS {
                let world = Arc::clone(&world);
                let cam = Arc::clone(&cam);
                let color_f64 = Arc::clone(&color_f64);
                let handle = thread::spawn(move || {
                    let u = (x as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v =
                        ((IMAGE_HEIGHT - y) as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let ray = cam.get_ray(u, v);
                    *color_f64.lock().unwrap() += ray.hit(&*world, MAX_DEPTH);
                });
                handles.push(handle);
            }
            handles.into_iter().for_each(|handle| {
                handle.join().unwrap();
            });
        }
        let color_f64 = color_f64.lock().unwrap();
        color_f64.to_color(SAMPLES_PER_IMAGE)
    });
    img.save("output.png").expect("Can't save image!");
}
