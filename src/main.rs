
#[macro_use]
extern crate interpolate;

use interpolate::curve::lagrange::Lagrange;

fn main() {
    let l = lagrange! {
        0.0  => 0.0,
        5.0  => 10.0,
        10.0 => 5.0,
        15.0 => 20.0,
    };

    for i in 0..100 {
        let x = 15.0 * (i as f32 / 100.0);
        let y = l.get(x);
        println!("{}, {}", x, y);
    }
}
