use num_traits::Float;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vector<F: Float, const N: usize, const P: isize>([F; N]);
pub type Displacement<F, const N: usize> = Vector<F, N, 0>;
pub type Position<F, const N: usize> = Vector<F, N, 1>;
pub type Color<F, const N: usize> = Displacement<F, N>;

impl<F: Float, const N: usize, const P: isize> Vector<F, N, P> {
    pub fn from_arr(arr: [F; N]) -> Self {
        Vector(arr)
    }
    pub fn new() -> Self {
        Self::from_arr([F::zero(); N])
    }
    pub fn arr(&self) -> &[F; N] {
        &self.0
    }
    pub fn arr_mut(&mut self) -> &mut [F; N] {
        &mut self.0
    }
}
impl<F: Float, const N: usize> Displacement<F, N> {
    pub fn dot(&self, other: &Self) -> F {
        let mut ans = F::zero();
        for i in 0..N {
            ans = ans + self.arr()[i] * other.arr()[i];
        }
        ans
    }

    pub fn norm_pow2(&self) -> F {
        Self::dot(&self, &self)
    }

    pub fn norm(&self) -> F {
        self.norm_pow2().sqrt()
    }

    pub fn unitize(self) -> Self {
        self / self.norm()
    }

    pub fn reflect(self, n: &Self) -> Self {
        self - *n * self.dot(n) - *n * self.dot(n)
    }
}

impl<F: Float, const N: usize, const P: isize> Add<Displacement<F, N>> for Vector<F, N, P> {
    type Output = Self;

    fn add(self, other: Displacement<F, N>) -> Self::Output {
        let mut ans = Self::Output::new();
        for i in 0..N {
            ans.0[i] = self.0[i] + other.0[i];
        }
        ans
    }
}

impl<F: Float, const N: usize, const P: isize> AddAssign<Displacement<F, N>> for Vector<F, N, P> {
    fn add_assign(&mut self, rhs: Displacement<F, N>) {
        *self = *self + rhs
    }
}

impl<F: Float, const N: usize, const P: isize> Sub<Displacement<F, N>> for Vector<F, N, P> {
    type Output = Self;

    fn sub(self, other: Displacement<F, N>) -> Self::Output {
        let mut ans = Self::Output::new();
        for i in 0..N {
            ans.0[i] = self.0[i] - other.0[i];
        }
        ans
    }
}

impl<F: Float, const N: usize> Sub<Self> for Position<F, N> {
    type Output = Displacement<F, N>;

    fn sub(self, other: Self) -> Self::Output {
        let mut ans = Self::Output::new();
        for i in 0..N {
            ans.0[i] = self.0[i] - other.0[i];
        }
        ans
    }
}

impl<F: Float, const N: usize> Mul<F> for Displacement<F, N> {
    type Output = Self;

    fn mul(self, other: F) -> Self::Output {
        Self(self.0.map(|t| t * other))
    }
}

impl<F: Float, const N: usize> Color<F, N> {
    pub fn mix(self, other: Self) -> Self {
        let mut ans = Self([F::zero(); N]);
        for i in 0..N {
            ans.0[i] = self.0[i] * other.0[i]
        }
        ans
    }
}

impl<F: Float, const N: usize> Div<F> for Displacement<F, N> {
    type Output = Self;

    fn div(self, other: F) -> Self::Output {
        Self(self.0.map(|t| t / other))
    }
}

impl<F: Float, const N: usize, const P: isize> Neg for Vector<F, N, P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.map(F::neg))
    }
}
