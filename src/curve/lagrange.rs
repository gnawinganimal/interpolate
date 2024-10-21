use num_traits::Num;
use std::cmp::e;
use super::curve::Curve;

pub struct Lagrange<T: Num + Copy> {
    points: Vec<(T, T)>,
}

impl<T: Num + Copy> Lagrange<T> {
    pub fn new() -> Self {
        Self {
            points: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn push(&mut self, x: T, y: T) {
        let j = self.points.binary_search_by(|(x_j, _)| x_j.cmp(&x));
        self.points.push((x, y));
    }

    pub fn basis(&self, j: usize, x: T) -> T {
        let mut num = T::one();
        let mut den = T::one();

        for i in 0..(self.len()) {
            if i == j {
                continue;
            }

            num = num * (x - self.points[i].0);
            den = den * (self.points[j].0 - self.points[i].0);
        }

        num / den
    }
}

impl<T: Num + Copy> Curve for Lagrange<T> {
    type T = T;
    
    fn get(&self, x: Self::T) -> Self::T {
        let mut y = T::zero();
    
        for j in 0..(self.points.len()) {
            y = y + (self.basis(j, x) * self.points[j].1);
        }

        y
    }
}
