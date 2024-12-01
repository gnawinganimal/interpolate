
#[macro_use]
pub mod polynomial;
pub mod vector;

#[cfg(test)]
mod tests {
    use super::polynomial::lagrange::Lagrange;

    #[test]
    fn x() {
        let c = lagrange! {
             0. =>  0.,
             5. => 10.,
            10. =>  0.,
        };

        for i in 0..100 {
            let x = 10.0 * (i as f32 / 100.0);
            let y = c.get(x);
            println!("{}, {}", x, y);
        }
    }
}
