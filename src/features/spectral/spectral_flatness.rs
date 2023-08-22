use crate::features::spectral::amplitude_spectrum;

pub fn compute(signal: &[f64]) -> f64 {
    let amplitude_spectrum: Vec<f64> = amplitude_spectrum::compute(signal);

    let partial = amplitude_spectrum.iter().fold((0.0, 0.0), |acc, &x| (acc.0 + x.ln(), acc.1 + x));

    ((partial.0 / (amplitude_spectrum.len() as f64)).exp() * (amplitude_spectrum.len() as f64)) /
        partial.1
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crate::utils::test;
    use std::f64;

    const FLOAT_PRECISION: f64 = 0.001_000_000;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let sf = compute(&dataset.signal);

        assert_relative_eq!(
            sf,
            dataset.features.spectralFlatness,
            epsilon = f64::EPSILON,
            max_relative = FLOAT_PRECISION
        );
    }

    #[test]
    fn test_spectral_flatness() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
