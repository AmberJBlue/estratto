pub fn compute(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }

    let mut prev_sample = signal[0];
    let mut zcr_count = 0.0;

    for &sample in &signal[1..] {
        if prev_sample.signum() != sample.signum() {
            zcr_count += 1.0;
        }
        prev_sample = sample;
    }

    zcr_count
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crate::utils::test;
    use std::f64;

    const FLOAT_PRECISION: f64 = 0.000_000_010;

    fn test_against(dataset: &test::data::TestDataSet) {
        let zcr = compute(&dataset.signal);
        assert_relative_eq!(
            zcr,
            dataset.features.zcr as f64,
            epsilon = f64::EPSILON,
            max_relative = FLOAT_PRECISION
        );
    }

    #[test]
    fn test_zcr() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
