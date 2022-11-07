use image::{ImageBuffer, RgbImage};
use std::rc::Rc;

mod hittable;
mod material;
mod ray;
mod solid_figures;
mod vector;

use hittable::{HitRecord, Hittable, HittableList};
use material::Material;
use rand::prelude::*;
use solid_figures::*;

fn main() {
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = 270;
    const SAMPLES_PER_IMAGE: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World

    let material_ground = Rc::new(Lambertian {
        albedo: Color3::from_arr([0.8, 0.8, 0.0]),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color3::from_arr([0.7, 0.3, 0.3]),
    });
    let material_left = Rc::new(Metal {
        albedo: Color3::from_arr([0.8, 0.8, 0.8]),
        fuzz: 0.0,
    });
    let material_right = Rc::new(Metal {
        albedo: Color3::from_arr([0.8, 0.6, 0.2]),
        fuzz: 0.0,
    });

    let world = HittableList {
        objects: vec![
            Rc::new(Sphere::new(
                Position3::from_arr([0.0, -100.5, -1.0]),
                100.0,
                material_ground,
            )),
            Rc::new(Sphere::new(
                Position3::from_arr([0.0, 0.0, -1.0]),
                0.5,
                material_center,
            )),
            Rc::new(Sphere::new(
                Position3::from_arr([-1.0, 0.0, -1.0]),
                0.5,
                material_left,
            )),
            Rc::new(Sphere::new(
                Position3::from_arr([1.0, 0.0, -1.0]),
                0.5,
                material_right,
            )),
        ],
    };

    // Camera

    let cam = Camera::new();

    // Render

    let img: RgbImage = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let mut color_f64 = Color3::new();
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
    origin: Position3<f64>,
    lower_left_corner: Position3<f64>,
    horizontal: Displacement3<f64>,
    vertical: Displacement3<f64>,
}

impl Camera {
    fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = 16.0 / 9.0 * viewport_height;
        let focal_length = 1.0;

        let origin = Position3::new();
        let horizontal = Displacement3::from_arr([viewport_width, 0.0, 0.0]);
        let vertical = Displacement3::from_arr([0.0, viewport_height, 0.0]);
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Displacement3::from_arr([0.0, 0.0, focal_length]);
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
    fn hit(&self, world: &dyn Hittable<f64, 3>, depth: i32) -> Color3<f64> {
        if depth <= 0 {
            return Color3::new();
        }
        if let Some(rec) = world.hit_by(self, 0.001..f64::INFINITY) {
            if let Some((scattered, attenuation)) =
                rec.material.upgrade().unwrap().scatter(self, &rec)
            {
                scattered.hit(world, depth - 1).mix(attenuation)
            } else {
                Color3::new()
            }
        } else {
            let u = self.direction.unitize();
            let t = 0.5 * (u.arr()[1] + 1.0);
            Color3::from_arr([
                ((1.0 - t) + t * 0.5),
                ((1.0 - t) + t * 0.7),
                ((1.0 - t) + t * 1.0),
            ])
        }
    }
}

impl Displacement3<f64> {
    fn to_color(self, samples_per_pixel: u32) -> image::Rgb<u8> {
        image::Rgb(
            self.arr()
                .map(|f| ((f / samples_per_pixel as f64).sqrt().clamp(0.0, 0.999) * 255.999) as u8),
        )
    }

    fn new_random_in_unit() -> Self {
        let mut ans = Self::new();
        loop {
            for f in ans.arr_mut() {
                *f = random();
            }
            if ans.norm_pow2() < 1.0 {
                return ans;
            }
        }
    }

    fn new_random_unit() -> Self {
        Self::new_random_in_unit().unitize()
    }

    fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        for e in self.arr() {
            if e.abs() >= S {
                return false;
            }
        }
        true
    }
}

struct Lambertian {
    albedo: Color3<f64>,
}

impl Material<f64, 3> for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray3<f64>,
        rec: &HitRecord<f64, 3>,
    ) -> Option<(Ray3<f64>, Color3<f64>)> {
        let mut scatter_direction = rec.normal + Displacement3::new_random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((
            Ray3 {
                origin: rec.p,
                direction: scatter_direction,
            },
            self.albedo,
        ))
    }
}

struct Metal {
    albedo: Color3<f64>,
    fuzz: f64,
}

impl Material<f64, 3> for Metal {
    fn scatter(&self, r_in: &Ray3<f64>, rec: &HitRecord3<f64>) -> Option<(Ray3<f64>, Color3<f64>)> {
        let reflected = r_in.direction.unitize().reflect(&rec.normal);
        let scattered = Ray3 {
            origin: rec.p,
            direction: reflected + Displacement3::new_random_in_unit() * self.fuzz,
        };
        if Displacement3::dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
