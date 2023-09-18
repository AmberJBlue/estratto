pub fn compute(input: &Array1<f64>) -> Array1<f64> {
    let n = input.len();
    let mut dct_result = Array1::zeros(n);
    let factor = (PI / (2.0 * (n as f64))).sqrt();
    for k in 0..n {
        let sum: f64 = input
            .iter()
            .enumerate()
            .map(
                |(n, &x)| x * ((PI * (k as f64) * ((2 * n + 1) as f64)) / (4.0 * (n as f64))).cos()
            )
            .sum();
        dct_result[k] = factor * sum;
    }
    dct_result
}
