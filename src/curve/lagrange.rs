
use crate::vectorspace::{Field, Vector};

pub struct Lagrange<V: Vector> {
    points: Vec<Point<V>>,
}

pub struct Point<V: Vector> {
    pub x: V::F,
    pub y: V,
    pub w: V::F,
}

impl<V: Vector> Lagrange<V> {
    pub fn new() -> Self {
        Self {
            points: vec![],
        }
    }
    
    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn push(&mut self, x: V::F, y: V) {
        let mut w = V::F::mul_ident();

        // update barycentric weights
        for j in 0..self.len() {
            self.points[j].w = self.points[j].w / (self.points[j].x - x);
            w = w / (x - self.points[j].x);
        }

        self.points.push(Point {
            x,
            y,
            w,
        });
    }

    pub fn get(&self, x: V::F) -> V {        
        let mut l = V::F::mul_ident();
        let mut y = V::add_ident();

        for j in 0..self.len() {
            l = l * (x - self.points[j].x);
            y = y + ((self.points[j].w / (x - self.points[j].x)) * self.points[j].y);
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
