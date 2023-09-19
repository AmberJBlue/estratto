extern crate ndarray;
use crate::features::spectral::amplitude_spectrum;
use crate::features::spectral::spectral_centroid;
use crate::core::fftfreq;
use ndarray::Array1;

pub fn compute(signal: &[f64]) -> f64 {
    let order = 2.0;
    let amplitude_spectrum: Vec<f64> = amplitude_spectrum::compute(&signal);
    let amp_spec = Array1::from(amplitude_spectrum.clone());
    let centroid = spectral_centroid::compute(&amplitude_spectrum);
    // Assuming 22050 sample rate and 2048 FFT size
    let freqs: Array1<f64> = Array1::from(fftfreq(22050, 2048));

    let deviation = &freqs - centroid; // Broadcast subtraction
    let deviation = deviation.mapv(f64::abs).mapv(|x| x.powf(order));

    let bandwidth = amp_spec.dot(&deviation).powf(1.0 / order);

    bandwidth
}
#[cfg(test)]
mod tests {
    // use super::*;
    // use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_spectral_bandwidth() {
        // let signal: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> =
        // let computed_bandwidth = compute(signal.as_slice().unwrap());
        // let expected_bandwidth = 10.0;

        // assert_approx_eq!(computed_bandwidth, expected_bandwidth, 1e-6);

    }
}
