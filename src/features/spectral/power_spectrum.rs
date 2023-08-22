use crate::features::spectral::amplitude_spectrum;

pub fn compute(signal: &[f64]) -> Vec<f64> {
    let amplitude_spectrum: Vec<f64> = amplitude_spectrum::compute(signal);

    let power_spectrum: Vec<f64> = amplitude_spectrum
        .iter()
        .map(|bin| bin.powi(2))
        .collect();

    power_spectrum
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crate::utils::test;
    use std::f64;

    const FLOAT_PRECISION: f64 = 0.333_333;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let power_spec = compute(&dataset.signal);

        test::data::approx_compare_vec(
            &power_spec,
            &dataset.features.powerSpectrum,
            FLOAT_PRECISION
        );
    }

    #[test]
    fn test_power_spectrum() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
