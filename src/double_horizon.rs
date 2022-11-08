use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::solid_figures::{Color3, Displacement3, HitRecord3, Position3, Ray3};

#[derive(Debug)]
pub(crate) struct Camera {
    origin: Position3<f64>,
    lower_left_corner: Position3<f64>,
    horizontal: Displacement3<f64>,
    vertical: Displacement3<f64>,
}

impl Camera {
    pub(crate) fn new(
        look_from: Position3<f64>,
        look_at: Position3<f64>,
        v_up: Displacement3<f64>,
        v_f_o_f: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = v_f_o_f.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unitize();
        let u = Displacement3::cross(&v_up, &w).unitize();
        let v = Displacement3::cross(&w, &u);

        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub(crate) fn get_ray(&self, s: f64, t: f64) -> Ray3<f64> {
        Ray3::new(
            self.origin,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        )
    }
}

impl Ray3<f64> {
    pub(crate) fn hit(&self, world: &dyn Hittable<f64, 3>, depth: i32) -> Color3<f64> {
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
    pub(crate) fn to_color(self, samples_per_pixel: u32) -> image::Rgb<u8> {
        image::Rgb(
            self.arr()
                .map(|f| ((f / samples_per_pixel as f64).sqrt().clamp(0.0, 0.999) * 255.999) as u8),
        )
    }

    fn new_random_in_unit() -> Self {
        let mut ans = Self::new();
        loop {
            for f in ans.arr_mut() {
                *f = rand::random();
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

pub(crate) struct Lambertian {
    albedo: Color3<f64>,
}

impl Lambertian {
    pub(crate) fn new(albedo: Color3<f64>) -> Self {
        Self { albedo }
    }
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

        Some((Ray3::new(rec.p, scatter_direction), self.albedo))
    }
}

pub(crate) struct Metal {
    albedo: Color3<f64>,
    fuzz: f64,
}

impl Metal {
    pub(crate) fn new(albedo: Color3<f64>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material<f64, 3> for Metal {
    fn scatter(&self, r_in: &Ray3<f64>, rec: &HitRecord3<f64>) -> Option<(Ray3<f64>, Color3<f64>)> {
        let reflected = r_in.direction.unitize().reflect(&rec.normal);
        let scattered = Ray3::new(
            rec.p,
            reflected + Displacement3::new_random_in_unit() * self.fuzz,
        );
        if Displacement3::dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub(crate) struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub(crate) fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0.powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material<f64, 3> for Dielectric {
    fn scatter(&self, r_in: &Ray3<f64>, rec: &HitRecord3<f64>) -> Option<(Ray3<f64>, Color3<f64>)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = r_in.direction.unitize();
        let cos_theta = f64::min(Displacement3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random() {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal, refraction_ratio)
            };

        Some((Ray3::new(rec.p, direction), Color3::from_arr([1.0; 3])))
    }
}
