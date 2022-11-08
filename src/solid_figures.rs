use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::*;
use crate::vector::*;
use num_traits::Float;
use std::ops::Range;
use std::sync::Arc;

pub(crate) type Displacement3<F> = Displacement<F, 3>;
pub(crate) type Position3<F> = Position<F, 3>;
pub(crate) type Color3<F> = Color<F, 3>;

pub(crate) type Ray3<F> = Ray<F, 3>;

pub(crate) type HitRecord3<F> = HitRecord<F, 3>;

impl<F: Float> Displacement3<F> {
    #[allow(dead_code)]
    pub fn cross(&self, other: &Self) -> Self {
        Self::from_arr([
            self.arr()[1] * other.arr()[2] - self.arr()[2] - other.arr()[1],
            self.arr()[2] * other.arr()[0] - self.arr()[0] - other.arr()[2],
            self.arr()[0] * other.arr()[1] - self.arr()[1] - other.arr()[0],
        ])
    }
}

pub(crate) struct Sphere<F: Float> {
    center: Position3<F>,
    radius: F,

    material: Arc<dyn Material<F, 3> + Send + Sync>,
}

impl<F: Float> Sphere<F> {
    pub fn new(
        center: Position3<F>,
        radius: F,
        material: Arc<dyn Material<F, 3> + Send + Sync>,
    ) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<F> Hittable<F, 3> for Sphere<F>
where
    F: Float,
{
    fn hit_by(&self, ray: &Ray3<F>, t_range: Range<F>) -> Option<HitRecord3<F>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_pow2();
        let half_b = Displacement3::dot(&oc, &ray.direction);
        let c = oc.norm_pow2() - self.radius.powi(2);

        let half_discriminant = half_b.powi(2) - a * c;
        if half_discriminant < F::zero() {
            return None;
        }
        let sqrt_half_discriminant = half_discriminant.sqrt();

        let mut root = (-half_b - sqrt_half_discriminant) / a;
        if !t_range.contains(&root) {
            root = (-half_b + sqrt_half_discriminant) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let mut rec = HitRecord3::new(p, t, &self.material);
        let outward_normal = (p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        Some(rec)
    }
}
