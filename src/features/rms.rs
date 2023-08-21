pub fn compute(signal: &[f64]) -> f64 {
    let sum = signal.iter().fold(0_f64, |acc, &sample| acc + sample.powi(2));
    let mean = sum / (signal.len() as f64);

    mean.sqrt()
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crate::utils::test;
    use approx::assert_relative_eq; // Add the approx crate for floating-point comparisons

    const FLOAT_PRECISION: f64 = 0.000_000_010;

    fn test_against(signal: &[f64], expected_rms: f64) {
        let rms = compute(signal);
        assert_relative_eq!(
            rms,
            expected_rms,
            epsilon = f64::EPSILON,
            max_relative = FLOAT_PRECISION
        );
    }

    #[test]
    fn test_rms() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(&dataset.signal, dataset.features.rms);
        }
    }
}
