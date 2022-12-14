use image::{ImageBuffer, RgbImage};
use std::sync::{Arc, Mutex};
use std::thread;

mod double_horizon;
mod hittable;
mod material;
mod ray;
mod solid_figures;
mod vector;

use crate::hittable::Hittable;
use crate::solid_figures::{Color3, Displacement3, Position3, Sphere};
use double_horizon::{Camera, Dielectric, Lambertian, Metal};
use hittable::HittableList;

const IMAGE_WIDTH: u32 = 480;
const IMAGE_HEIGHT: u32 = 270;
const SAMPLES_PER_IMAGE: u32 = 96;
const THREADS: u32 = 8;
const MAX_DEPTH: i32 = 48;

fn main() {
    // World

    let world = get_scene();

    // Camera

    let look_from = Position3::from_arr([3.0, 3.0, 2.0]);
    let look_at = Position3::from_arr([0.0, 0.0, -1.0]);
    let v_up = Displacement3::from_arr([0.0, 1.0, 0.0]);
    let dist_to_focus = (look_from - look_at).norm();
    let aperture = 2.0;
    let cam = Arc::new(Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64,
        aperture,
        dist_to_focus,
    ));

    // Render

    rend(&world, &cam, "output.png");
}

fn get_scene() -> Arc<dyn Hittable<f64, 3> + Send + Sync> {
    let material_ground = Arc::new(Lambertian::new(Color3::from_arr([0.8, 0.8, 0.0])));
    let material_center = Arc::new(Lambertian::new(Color3::from_arr([0.1, 0.2, 0.5])));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color3::from_arr([0.8, 0.6, 0.2]), 0.0));

    Arc::new(HittableList {
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
                material_left.clone(),
            )),
            Arc::new(Sphere::new(
                Position3::from_arr([-1.0, 0.0, -1.0]),
                -0.45,
                material_left,
            )),
            Arc::new(Sphere::new(
                Position3::from_arr([1.0, 0.0, -1.0]),
                0.5,
                material_right,
            )),
        ],
    })
}

fn rend(
    world: &Arc<dyn Hittable<f64, 3> + Send + Sync>,
    cam: &Arc<Camera>,
    output_path: impl AsRef<str>,
) {
    let img: RgbImage = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let color_f64 = Arc::new(Mutex::new(Color3::new()));
        (0..THREADS)
            .map(|_| {
                let world = Arc::clone(world);
                let cam = Arc::clone(cam);
                let color_f64 = Arc::clone(&color_f64);
                thread::spawn(move || {
                    for _ in 0..SAMPLES_PER_IMAGE / THREADS {
                        let u = (x as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                        let v = ((IMAGE_HEIGHT - y) as f64 + rand::random::<f64>())
                            / (IMAGE_HEIGHT - 1) as f64;
                        let ray = cam.get_ray(u, v);
                        let color_f64_sample = ray.hit(&*world, MAX_DEPTH);
                        *color_f64.lock().unwrap() += color_f64_sample;
                    }
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|handle| {
                handle.join().unwrap();
            });
        let color_f64 = color_f64.lock().unwrap();
        color_f64.to_color(SAMPLES_PER_IMAGE)
    });
    img.save(output_path.as_ref()).expect("Can't save image!");
}
