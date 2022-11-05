use num_traits::Float;
use crate::vector::Vector;

pub struct Ray<F: Float, const N: usize> {
    pub origin: Vector<F, N>,
    pub direction: Vector<F, N>,
}
impl<F: Float, const N: usize> Ray<F, N> {
    pub fn at(&self, t: F) -> Vector<F, N> {
        self.origin + self.direction * t
    }
}