use rustfft::{ FftPlanner, num_complex::Complex, num_traits::Zero, FftDirection };

/// Computes the FFT of a real-valued signal.
///
/// # Arguments
///
/// * `signal` - A slice of real-valued input samples.
///
/// # Returns
///
/// A vector of complex-valued frequency-domain samples.
pub fn compute_fft(signal: &[f32]) -> Vec<Complex<f32>> {
    // Create a FFT planner for the desired FFT size.
    let fft_size = signal.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);

    // Allocate input and output buffers.
    let mut input: Vec<Complex<f32>> = signal
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    let output: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); fft_size];

    // Execute the FFT.
    fft.process(&mut input);

    output
}

/// Performs an inverse FFT on a complex input signal, returning the resulting real output signal.
///
/// # Arguments
///
/// * `signal` - A slice of complex input samples.
///
/// # Returns
///
/// A vector containing the resulting real output samples.
pub fn ifft(signal: &[Complex<f32>]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(signal.len());

    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); signal.len()];
    output.copy_from_slice(signal);

    fft.process(&mut output);

    output
        .iter()
        .map(|c| c.re)
        .collect()
}

/// Computes the power spectrum of a complex input signal.
///
/// # Arguments
///
/// * `signal` - A slice of complex input samples.
///
/// # Returns
///
/// A vector containing the resulting power spectrum.
pub fn power_spectrum(signal: &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft(signal.len(), FftDirection::Forward);
    let mut input: Vec<Complex<f32>> = signal
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    let output: Vec<Complex<f32>> = vec![Complex::zero(); signal.len()];

    fft.process(&mut input);

    let mut power_spectrum: Vec<f32> = output
        .iter()
        .map(|c| c.norm_sqr() / (signal.len() as f32))
        .collect();
    power_spectrum[0] = 0.0;
    power_spectrum.truncate(signal.len() / 2);
    power_spectrum
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_compute_fft() {
//         // Create a test signal containing a single sine wave with frequency 440 Hz
//         let sample_rate = 44100;
//         let duration = 1.0;
//         let num_samples = ((sample_rate as f32) * duration) as usize;
//         let frequency = 440.0;
//         let amplitude = 0.5;
//         let mut samples = Vec::with_capacity(num_samples);
//         for n in 0..num_samples {
//             let t = (n as f32) / (sample_rate as f32);
//             let x = amplitude * (2.0 * std::f32::consts::PI * frequency * t).sin();
//             samples.push(x);
//         }

//         // Compute the FFT of the test signal
//         let fft_result = compute_fft(&samples);

//         // Check that the length of the FFT result matches the expected length
//         assert_eq!(fft_result.len(), num_samples);

//         // Check that the first element of the FFT result is close to zero (DC offset)
//         let tolerance = 0.75;
//         assert!(fft_result[0].norm() < tolerance);

//         // Check that the second element of the FFT result is close to the amplitude of the input signal
//         assert!((fft_result[1].re - amplitude).abs() < tolerance);
//     }

//     #[test]
//     fn test_power_spectrum() {
//         // Create a test signal containing a single sine wave with frequency 440 Hz
//         let sample_rate = 44100;
//         let duration = 1.0;
//         let num_samples = ((sample_rate as f32) * duration) as usize;
//         let frequency = 440.0;
//         let amplitude = 0.5;
//         let mut samples = Vec::with_capacity(num_samples);
//         for n in 0..num_samples {
//             let t = (n as f32) / (sample_rate as f32);
//             let x = amplitude * (2.0 * std::f32::consts::PI * frequency * t).sin();
//             samples.push(x);
//         }

//         // Compute the power spectrum of the test signal
//         let power_spectrum_result = power_spectrum(&samples);

//         // Check that the length of the power spectrum result matches the expected length
//         assert_eq!(power_spectrum_result.len(), num_samples / 2);

//         // Check that the first element of the power spectrum result is close to zero (DC offset)
//         let tolerance = 100.0;
//         assert!(power_spectrum_result[0] < tolerance);

//         // Check that the second element of the power spectrum result is close to the amplitude of the input signal
//         assert!(
//             (power_spectrum_result[1] - (amplitude.powi(2) * (num_samples as f32)) / 2.0).abs() <
//                 tolerance
//         );
//     }
// }