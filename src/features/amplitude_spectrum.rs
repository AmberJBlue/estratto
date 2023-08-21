use rustfft::{ num_complex::Complex, FftPlanner };

pub fn compute(signal: &[f64]) -> Vec<f64> {
    let fft_len = signal.len();
    let fft = FftPlanner::new().plan_fft_forward(fft_len);

    // Convert the input slice to a vector of complex numbers. This step is necessary
    // because FFT operations are inherently complex-valued.
    let mut fft_buffer: Vec<Complex<f64>> = signal
        .iter()
        .map(|&sample| Complex::new(sample, 0_f64))
        .collect();

    fft.process(&mut fft_buffer);

    let amplitude_spectrum: Vec<f64> = fft_buffer
        .iter()
        .take(fft_len / 2)
        .map(|bin| {
            let tmp = bin.re.powf(2_f64) + bin.im.powf(2_f64);
            tmp.sqrt()
        })
        .collect();

    amplitude_spectrum
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crate::utils::test;
    use std::f64;

    const FLOAT_PRECISION: f64 = 0.333_333;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        // This borrows the dataset.signal vector as a slice.
        let amp_spec = compute(&dataset.signal[..]);

        test::data::approx_compare_vec(
            &amp_spec,
            &dataset.features.amplitudeSpectrum,
            FLOAT_PRECISION
        );
    }

    #[test]
    fn test_amplitude_spectrum() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
