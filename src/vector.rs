use num_traits::Float;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Vector<F: Float, const N: usize, const P: isize>([F; N]);
pub(crate) type Displacement<F, const N: usize> = Vector<F, N, 0>;
pub(crate) type Position<F, const N: usize> = Vector<F, N, 1>;
pub(crate) type Color<F, const N: usize> = Displacement<F, N>;

impl<F: Float, const N: usize, const P: isize> Vector<F, N, P> {
    pub(crate) fn from_arr(arr: [F; N]) -> Self {
        Vector(arr)
    }
    pub(crate) fn new() -> Self {
        Self::from_arr([F::zero(); N])
    }
    pub(crate) fn arr(&self) -> &[F; N] {
        &self.0
    }
    pub(crate) fn arr_mut(&mut self) -> &mut [F; N] {
        &mut self.0
    }
}
impl<F: Float, const N: usize> Displacement<F, N> {
    pub(crate) fn dot(&self, other: &Self) -> F {
        let mut ans = F::zero();
        for i in 0..N {
            ans = ans + self.arr()[i] * other.arr()[i];
        }
        ans
    }

    pub(crate) fn norm_pow2(&self) -> F {
        Self::dot(self, self)
    }

    pub(crate) fn norm(&self) -> F {
        self.norm_pow2().sqrt()
    }

    pub(crate) fn unitize(self) -> Self {
        self / self.norm()
    }

    pub(crate) fn reflect(self, n: &Self) -> Self {
        self - *n * Self::dot(&self, n) - *n * Self::dot(&self, n)
    }

    pub(crate) fn refract(self, n: &Self, etai_over_etat: F) -> Self {
        let cos_theta = F::min(Self::dot(&-self, n), F::one());
        let r_out_perp = (self + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = *n * -(F::one() - r_out_perp.norm_pow2()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl<F: Float, const N: usize, const P: isize> Add<Displacement<F, N>> for Vector<F, N, P> {
    type Output = Self;

    fn add(self, other: Displacement<F, N>) -> Self::Output {
        let mut ans = Self::Output::new();
        for i in 0..N {
            ans.arr_mut()[i] = self.arr()[i] + other.arr()[i];
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
            ans.arr_mut()[i] = self.arr()[i] - other.arr()[i];
        }
        ans
    }
}

impl<F: Float, const N: usize> Sub<Self> for Position<F, N> {
    type Output = Displacement<F, N>;

    fn sub(self, other: Self) -> Self::Output {
        let mut ans = Self::Output::new();
        for i in 0..N {
            ans.arr_mut()[i] = self.arr()[i] - other.arr()[i];
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
    pub(crate) fn mix(self, other: Self) -> Self {
        let mut ans = Self([F::zero(); N]);
        for i in 0..N {
            ans.arr_mut()[i] = self.arr()[i] * other.arr()[i]
        }
        ans
    }
}

impl<F: Float, const N: usize> Div<F> for Displacement<F, N> {
    type Output = Self;

    fn div(self, other: F) -> Self::Output {
        Self(self.arr().map(|t| t / other))
    }
}

impl<F: Float, const N: usize, const P: isize> Neg for Vector<F, N, P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.arr().map(F::neg))
    }
}
