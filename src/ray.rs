use num_traits::Float;
use crate::vector::{Displacement, Position};

pub struct Ray<F: Float, const N: usize> {
    pub origin: Position<F, N>,
    pub direction: Displacement<F, N>,
}
impl<F: Float, const N: usize> Ray<F, N> {
    pub fn at(&self, t: F) -> Position<F, N> {
        self.origin + self.direction * t
    }
}