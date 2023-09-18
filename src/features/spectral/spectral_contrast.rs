use crate::features::spectral::amplitude_spectrum;

pub fn compute(signal: &[f64]) -> Vec<f64> {
    let sub_bands = vec![0..2, 2..4, 4..7];

    let mut values = Vec::with_capacity(sub_bands.len());

    for range in &sub_bands {
        let sub_bands_data: Vec<f64> = signal[range.clone()].to_vec();

        let peak = *sub_bands_data
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        let valley = *sub_bands_data
            .iter()
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();

        values.push(peak - valley);
    }

    values
}

#[cfg(test)]
mod tests {
    use super::compute;
    #[test]
    fn test_spectral_contrast() {
        let amplitude_spectrum = [1.0, 2.0, 4.0, 8.0, 10.0, 20.0, 30.0];
        let result = compute(&amplitude_spectrum);

        // The expected contrast values based on the provided amplitude spectrum are:
        // Sub-band 1: 2.0 - 1.0 = 1.0
        // Sub-band 2: 8.0 - 4.0 = 4.0
        // Sub-band 3: 30.0 - 10.0 = 20.0
        let expected = vec![1.0, 4.0, 20.0];

        assert_eq!(result, expected);
    }
}
