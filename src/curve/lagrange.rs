use num_traits::Num;

pub struct Lagrange<T: Num + Copy> {
    points: Vec<Point<T>>,
}

pub struct Point<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub w: T,
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
        let mut w = T::one();

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

    pub fn get(&self, x: T) -> T {        
        let mut l = T::one();
        let mut s = T::zero();

        for j in 0..self.len() {
            l = l * (x - self.points[j].x);
            s = s + (self.points[j].y * self.points[j].w / (x - self.points[j].x));
        }

        l * s
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
