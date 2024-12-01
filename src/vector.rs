use std::ops::{Add, Div, Mul, Sub};

use num_traits::Num;

pub trait Vector: Sized + Add<Output = Self> + Sub<Output = Self> {
    type F: Copy + Num + Mul<Self, Output = Self>;

    fn zero() -> Self;
}

impl<F: Copy + Num + Mul<Self, Output = Self>> Vector for F {
    type F = F;

    fn zero() -> Self {
        F::zero()
    }
}
