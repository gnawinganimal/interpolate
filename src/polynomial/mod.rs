use crate::vector::Vector;

#[macro_use]
pub mod lagrange;

pub trait Polynomail<V: Vector> {
    fn len(&self) -> usize;

    fn get(&self, x: V::F) -> V;

    fn point(&self, i: usize) -> Point<V>;
}

#[derive(Clone, Copy)]
pub struct Point<V: Vector> {
    pub x: V::F,
    pub y: V,
}

