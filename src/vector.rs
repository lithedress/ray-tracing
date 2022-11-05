use num_traits::Float;
use std::ops::{Add, Sub, Mul, Div, Index, Neg, AddAssign};

#[derive(Clone, Copy)]
pub struct Vector<F: Float, const N: usize>([F; N]);

impl<F: Float, const N: usize> Vector<F, N> {
    pub fn from_arr(arr: [F; N]) -> Self {
        Self(arr)
    }
    pub fn dot(&self, other: &Self) -> F {
        let mut ans = F::zero();
        for i in 0..N {
            ans = ans + self.0[i] * other.0[i];
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

    pub fn map<Fc, U>(self, f: Fc) -> [U; N] where Fc: FnMut(F) -> U {
        self.0.map(f)
    }
}

impl<F: Float, const N: usize> Add<Self> for Vector<F, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut ans = Self([F::zero(); N]);
        for i in 0..N {
            ans.0[i] = self.0[i] + other.0[i];
        }
        ans
    }
}

impl<F: Float, const N: usize> AddAssign<Self> for Vector<F, N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<F: Float, const N: usize> Sub<Self> for Vector<F, N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut ans = Self([F::zero(); N]);
        for i in 0..N {
            ans.0[i] = self.0[i] - other.0[i];
        }
        ans
    }
}

impl<F: Float, const N: usize> Mul<F> for Vector<F, N> {
    type Output = Self;

    fn mul(self, other: F) -> Self::Output {
        Self(
            self.0.map(|t| t * other)
        )
    }
}

impl<F: Float, const N: usize> Div<F> for Vector<F, N> {
    type Output = Self;

    fn div(self, other: F) -> Self::Output {
        Self(
            self.0.map(|t| t / other)
        )
    }
}

impl<F: Float, const N: usize> Index<usize> for Vector<F, N> {
    type Output = F;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<F: Float, const N: usize> Neg for Vector<F, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.map(F::neg))
    }
}