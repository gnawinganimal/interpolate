use std::ops::{Add, Div, Mul, Sub};

pub trait Field: Sized + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> {
    fn add_ident() -> Self;

    fn mul_ident() -> Self;
}

impl Field for f32 {
    fn add_ident() -> Self {
        0.0
    }

    fn mul_ident() -> Self {
        1.0
    }
}

impl Field for f64 {
    fn add_ident() -> Self {
        0.0
    }

    fn mul_ident() -> Self {
        1.0
    }
}

pub trait Vector: Sized + Copy + Add<Output = Self> + Sub<Output = Self> {
    type F: Field + Mul<Self, Output = Self>;

    fn add_ident() -> Self;
}

impl<F: Field> Vector for F {
    type F = F;

    fn add_ident() -> Self {
        F::add_ident()
    }
}
