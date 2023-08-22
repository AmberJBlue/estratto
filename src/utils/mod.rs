pub mod test;
pub type Hz = f64;

pub fn mu(exp: i32, vector: &[f64]) -> f64 {
    let fraction = vector
        .iter()
        .enumerate()
        .fold((0.0, 0.0), |acc, x| {
            let num_inc = (x.0 as f64).powi(exp) * x.1.abs();
            let den_inc = x.1;

            (acc.0 + num_inc, acc.1 + den_inc)
        });

    fraction.0 / fraction.1
}
