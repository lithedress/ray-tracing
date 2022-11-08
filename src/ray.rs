use crate::vector::{Displacement, Position};
use num_traits::Float;

pub(crate) struct Ray<F: Float, const N: usize> {
    pub(crate) origin: Position<F, N>,
    pub(crate) direction: Displacement<F, N>,
}
impl<F: Float, const N: usize> Ray<F, N> {
    pub fn new(origin: Position<F, N>, direction: Displacement<F, N>) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: F) -> Position<F, N> {
        self.origin + self.direction * t
    }
}
