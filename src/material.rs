use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::Color;
use num_traits::Float;

pub trait Material<F: Float, const N: usize> {
    fn scatter(&self, r_in: &Ray<F, N>, rec: &HitRecord<F, N>) -> Option<(Ray<F, N>, Color<F, N>)>;
}
