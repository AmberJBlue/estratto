use crate::utils::correlation::autocorrelate;
use crate::utils::fft;

// This function extracts the pitch of an audio sample using various pitch detection algorithms.
//
// # Arguments
//
// * `samples` - A slice of audio samples.
// * `sample_rate` - The sample rate of the audio.
// * `algo` - A string specifying the pitch detection algorithm to use. Valid values are "autocorrelation",
//            "yin", and "hps". If no algorithm is specified, "autocorrelation" is used by default.
//
// # Returns
//
// The pitch of the audio sample as a floating point number.
pub fn extract_pitch(samples: &[f32], sample_rate: u32, algo: Option<&str>) -> f32 {
    match algo {
        Some("yin") => extract_pitch_yin(samples, sample_rate),
        Some("hps") => extract_pitch_hps(samples, sample_rate),
        _ => extract_pitch_autocorrelation(samples, sample_rate),
    }
}

/// Extracts the pitch of an audio sample using the autocorrelation algorithm.
///
/// # Arguments
///
/// * `samples` - A slice of audio samples.
/// * `sample_rate` - The sample rate of the audio.
///
/// # Returns
///
/// The pitch of the audio sample as a floating point number.
pub fn extract_pitch_autocorrelation(samples: &[f32], sample_rate: u32) -> f32 {
    let min_pitch = 80; // Hz
    let max_pitch = 1000; // Hz
    let buffer_size = ((sample_rate as f32) / (min_pitch as f32)) as usize;
    let autocorrelation = autocorrelate(samples, buffer_size);

    let mut pitch = 0.0;
    let mut max_peak = 0.0;

    let start = (min_pitch * buffer_size) / (sample_rate as usize);
    let end = (max_pitch * buffer_size) / (sample_rate as usize);

    if start >= autocorrelation.len() || end >= autocorrelation.len() {
        return 0.0;
    }

    for i in start..end {
        let peak = autocorrelation[i];
        if peak > max_peak {
            max_peak = peak;
            pitch = i as f32;
        }
    }

    if max_peak == 0.0 {
        return 0.0;
    }

    // Interpolate peak using parabolic interpolation
    let y1 = autocorrelation[(pitch - 1.0) as usize];
    let y2 = max_peak;
    let y3 = autocorrelation[(pitch + 1.0) as usize];
    let delta = (0.5 * (y1 - y3)) / (y1 - 2.0 * y2 + y3);
    let refined_pitch = pitch + delta;
    (refined_pitch * (sample_rate as f32)) / (buffer_size as f32)
}

/// Extracts the pitch of an audio sample using the YIN algorithm.
///
/// # Arguments
///
/// * `samples` - A slice of audio samples.
/// * `sample_rate` - The sample rate of the audio.
///
/// # Returns
///
/// The pitch of the audio sample as a floating point number.
pub fn extract_pitch_yin(samples: &[f32], sample_rate: u32) -> f32 {
    let sample_size = samples.len();
    let tau_min = ((sample_rate as f32) / 500.0) as usize;
    let tau_max = ((sample_rate as f32) / 50.0) as usize;

    let mut difference: Vec<f32> = vec![0.0; tau_max];
    let mut yin: Vec<f32> = vec![0.0; tau_max];

    for tau in tau_min..tau_max {
        difference[tau] = 0.0;
        for j in 0..sample_size - tau {
            difference[tau] += (samples[j] - samples[j + tau]).powi(2);
        }

        yin[tau] = difference[tau];

        if tau > 0 {
            yin[tau] += yin[tau - 1];
        }
    }

    let mut tau: usize = tau_min;
    while tau < tau_max - 1 && yin[tau] > 0.1 * yin[0] {
        tau += 1;
    }

    if tau == tau_max - 1 {
        return 0.0;
    }

    let s0 = yin[tau - 1];
    let s1 = yin[tau];
    let s2 = yin[tau + 1];

    let mut interp = (0.5 * (s2 - s0)) / (2.0 * s1 - s2 - s0);

    if interp.is_nan() {
        interp = 0.0;
    }

    if tau < 1 || tau >= yin.len() - 1 {
        return 0.0;
    }

    ((sample_rate as f32) / ((tau as f32) + interp)) as f32
}

/// Extracts the pitch of an audio sample using the HPS algorithm.
///
/// # Arguments
///
/// * `samples` - A slice of audio samples.
/// * `sample_rate` - The sample rate of the audio.
///
/// # Returns
///
/// The pitch of the audio sample as a floating point number.
pub fn extract_pitch_hps(samples: &[f32], sample_rate: u32) -> f32 {
    // The maximum fundamental frequency that we're interested in is the Nyquist frequency,
    // which is half the sample rate.
    let max_frequency = (sample_rate / 2) as usize;

    // Compute the power spectrum of the audio signal using the FFT algorithm.
    let power_spectrum = fft::power_spectrum(&samples);

    // Compute the harmonic product spectrum of the power spectrum.
    let mut hps = vec![0.0; power_spectrum.len()];
    for harmonic in 2..=4 {
        let mut resampled_spectrum = vec![0.0; max_frequency / harmonic];
        for i in 0..resampled_spectrum.len() {
            for j in 0..harmonic {
                resampled_spectrum[i] += power_spectrum[i * harmonic + j];
            }
        }
        for i in 0..resampled_spectrum.len() * harmonic {
            hps[i] *= resampled_spectrum[i / harmonic];
        }
    }

    // Find the index of the maximum value in the harmonic product spectrum.
    let max_index = hps
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0);

    // Convert the index of the maximum value to a frequency value in Hz.
    ((max_index as f32) * (sample_rate as f32)) / (max_frequency as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pitch_autocorrelation() {
        // Create a test signal containing a single sine wave with frequency 440 Hz
        let sample_rate = 44100;
        let duration = 1.0;
        let num_samples = ((sample_rate as f32) * duration) as usize;
        let frequency = 440.0;
        let amplitude = 0.5;
        let mut samples = Vec::with_capacity(num_samples);
        for n in 0..num_samples {
            let t = (n as f32) / (sample_rate as f32);
            let x = amplitude * (2.0 * std::f32::consts::PI * frequency * t).sin();
            samples.push(x);
        }

        // Extract the pitch from the test signal using the autocorrelation algorithm
        let pitch = extract_pitch(&samples, sample_rate, Some("autocorrelation"));

        // Check that the extracted pitch is close to the actual pitch
        let actual_pitch = frequency;
        let tolerance = 1.0; // Hz
        println!("pitch: {}", pitch);
        println!("actual_pitch: {}", actual_pitch);
        assert!(
            (pitch - actual_pitch).abs() < tolerance,
            "pitch: {}, actual_pitch: {}",
            pitch,
            actual_pitch
        );
    }

    // #[test]
    // fn test_extract_pitch_yin() {
    //     // Create a test signal containing a single sine wave with frequency 440 Hz
    //     let sample_rate = 44100;
    //     let duration = 1.0;
    //     let num_samples = ((sample_rate as f32) * duration) as usize;
    //     let frequency = 1568.0; // G6
    //     let amplitude = 0.5;
    //     let mut samples = Vec::with_capacity(num_samples);
    //     for n in 0..num_samples {
    //         let t = (n as f32) / (sample_rate as f32);
    //         let x = amplitude * (2.0 * std::f32::consts::PI * frequency * t).sin();
    //         samples.push(x);
    //     }

    //     // Extract the pitch from the test signal using the YIN algorithm
    //     let pitch = extract_pitch(&samples, sample_rate, Some("yin"));

    //     // Check that the extracted pitch is close to the actual pitch
    //     let actual_pitch = frequency;
    //     let tolerance = 1.0; // Hz
    //     assert!((pitch - actual_pitch).abs() < tolerance);
    // }

    // #[test]
    // fn test_extract_pitch_hps() {
    //     // Create a test signal containing a single sine wave with frequency 440 Hz
    //     let sample_rate = 44100;
    //     let duration = 1.0;
    //     let num_samples = ((sample_rate as f32) * duration) as usize;
    //     let frequency = 440.0;
    //     let amplitude = 0.5;
    //     let mut samples = Vec::with_capacity(num_samples);
    //     for n in 0..num_samples {
    //         let t = (n as f32) / (sample_rate as f32);
    //         let x = amplitude * (2.0 * std::f32::consts::PI * frequency * t).sin();
    //         samples.push(x);
    //     }

    //     // Extract the pitch from the test signal using the HPS algorithm
    //     let pitch = extract_pitch(&samples, sample_rate, Some("hps"));

    //     // Check that the extracted pitch is close to the actual pitch
    //     let actual_pitch = frequency;
    //     let tolerance = 5.0; // Hz
    //     assert!((pitch - actual_pitch).abs() < tolerance);
    // }

    #[test]
    fn test_extract_pitch_default() {
        // Create a test signal containing a single sine wave with frequency 440 Hz
        let sample_rate = 44100;
        let duration = 1.0;
        let num_samples = ((sample_rate as f32) * duration) as usize;
        let frequency = 440.0;
        let amplitude = 0.5;
        let mut samples = Vec::with_capacity(num_samples);
        for n in 0..num_samples {
            let t = (n as f32) / (sample_rate as f32);
            let x = amplitude * (2.0 * std::f32::consts::PI * frequency * t).sin();
            samples.push(x);
        }

        // Extract the pitch from the test signal using the HPS algorithm
        let pitch = extract_pitch(&samples, sample_rate, None);
        let auto_pitch = extract_pitch_autocorrelation(&samples, sample_rate);
        // Check that the extracted pitch is close to the actual pitch
        let actual_pitch = frequency;
        let tolerance = 1.0; // Hz
        assert!((pitch - actual_pitch).abs() < tolerance);
        assert!((auto_pitch - actual_pitch).abs() < tolerance);
        assert_eq!(pitch, auto_pitch);
    }
}