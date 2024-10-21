use num_traits::Num;

pub trait Curve {
    type T: Num + Clone + Copy;

    fn get(&self, x: Self::T) -> Self::T;
}
