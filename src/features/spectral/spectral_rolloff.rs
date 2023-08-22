use crate::features::spectral::amplitude_spectrum;

pub fn compute(signal: &[f64], sample_rate: f64, rolloff_point: Option<f64>) -> f64 {
    let amplitude_spectrum: Vec<f64> = amplitude_spectrum::compute(signal);

    let nyquist_bin = sample_rate / (2.0 * ((amplitude_spectrum.len() as f64) - 1.0));
    let mut integral: f64 = amplitude_spectrum.iter().sum();

    let threshold = match rolloff_point {
        Some(rfp) => rfp * integral,
        None => 0.99 * integral,
    };

    let mut iterator = (amplitude_spectrum.len() as i32) - 1;

    while integral > threshold && iterator >= 0 {
        integral -= amplitude_spectrum[iterator as usize];
        iterator -= 1;
    }

    ((iterator + 1) as f64) * nyquist_bin
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crate::utils::test;
    use std::f64;

    const FLOAT_PRECISION: f64 = 0.000_000_010;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let sr = compute(&dataset.signal, dataset.info.sampleRate as f64, None);

        assert_relative_eq!(
            sr,
            dataset.features.spectralRolloff,
            epsilon = f64::EPSILON,
            max_relative = FLOAT_PRECISION
        );
    }

    #[test]
    fn test_spectral_rolloff() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
