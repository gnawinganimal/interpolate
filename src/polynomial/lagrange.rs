
use crate::vector::Vector;
use num_traits::One;

use super::Point;

pub struct Lagrange<V: Copy + Vector> {
    points: Vec<(Point<V>, V::F)>,
}

pub struct WeightedPoint {

}

impl<V: Copy + Vector> Lagrange<V> {
    pub fn new() -> Self {
        Self {
            points: vec![],
        }
    }
    
    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn push(&mut self, x: V::F, y: V) {
        let mut w = V::F::one();

        // update barycentric weights
        for j in 0..self.len() {
            self.points[j].1 = self.points[j].1 / (self.points[j].0.x - x);
            w = w / (x - self.points[j].0.x);
        }

        self.points.push((Point { x, y }, w));
    }

    pub fn get(&self, x: V::F) -> V {        
        let mut l = V::F::one();
        let mut y = V::zero();

        for j in 0..self.len() {
            l = l * (x - self.points[j].0.x);
            y = y + ((self.points[j].1 / (x - self.points[j].0.x)) * self.points[j].0.y);
        }
        y = l * y;

        y
    }
}

#[macro_export]
macro_rules! lagrange {
    ($($x:expr => $y:expr),* $(,)?) => {{
        let mut c = Lagrange::new();

        $( c.push($x, $y); )*

        c
    }}
}
