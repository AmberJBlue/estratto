// Import required dependencies when testing
#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate approx;

// Internal modules
mod core;
mod filters;
mod features;
mod utils;

/// A type alias for Hertz, representing frequency in cycles per second.
pub type Hz = core::Hz;

/// Amplitude Spectrum
///
/// Also referred to as the magnitude spectrum, this is derived by executing the Fast
/// Fourier Transform (FFT) on a signal, converting it to its frequency representation.
/// The outcome is an array where each slot represents a frequency bin, holding a complex
/// number (both real and imaginary parts). The amplitude spectrum determines the amplitude
/// for each of these slots. This output showcases the signal's frequency components and their
/// respective intensities.
///
/// # Parameters
///
/// * `signal` - A slice of the signal samples.
///
/// # Returns
///
/// A vector containing the amplitude spectrum.
///
pub fn get_amplitude_spectrum(signal: &[f64]) -> Vec<f64> {
    features::spectral::amplitude_spectrum::compute(signal)
}

/// Power Spectrum
///
/// This is the square of the `amplitude_spectrum`
///
/// # Parameters
///
/// * `signal` - A reference to a vector of the signal samples.
///
/// # Returns
///
/// A vector containing the power spectrum.
///
pub fn get_power_spectrum(signal: &[f64]) -> Vec<f64> {
    features::spectral::power_spectrum::compute(signal)
}

/// Spectral Centroid
///
/// A measure signifying the sound's "brightness", illustrating the weighted average
/// of the frequency spectrum. Imagine converting the spectrum into a wooden plank
/// and attempting to balance it on a pivot point along the X axis. The frequency
/// at which this plank perfectly balances on the pivot symbolizes the spectral centroid.
///
/// # Parameters
///
/// * `signal` - A reference to a vector of the signal samples.
///
/// # Returns
///
/// A floating point value representing the spectral centroid.
///
pub fn get_spectral_centroid(signal: &[f64]) -> f64 {
    features::spectral::spectral_centroid::compute(signal)
}

/// Spectral Flatness
///
/// Measures the uniformity of a spectrum by comparing the geometric mean to the arithmetic mean.
/// A spectrum that is more noise-like (as opposed to being tonal) will have a higher spectral flatness.
///
/// # Parameters
///
/// * `signal` - A slice of the signal samples.
///
/// # Returns
///
/// A floating point value representing the spectral flatness of the signal, where a higher value indicates
/// a more uniform, noise-like spectrum, and a lower value indicates a more tonal spectrum.
///
pub fn get_spectral_flatness(signal: &[f64]) -> f64 {
    features::spectral::spectral_flatness::compute(signal)
}

/// Spectral Rolloff
///
/// The threshold frequency where a specified percentage of the spectrum's energy
/// resides below it.
/// # Parameters
///
/// * `signal` - A reference to a vector of the signal samples.
/// * `sample_rate` - The sample rate of the signal in Hz.
/// * `rolloff_point` - An optional threshold to compute the rolloff, defaults to 0.99 if not provided.
///
/// # Returns
///
/// A floating point value representing the spectral rolloff.
///
pub fn get_spectral_rolloff(signal: &[f64], sample_rate: f64, rolloff_point: Option<f64>) -> f64 {
    features::spectral::spectral_rolloff::compute(signal, sample_rate, rolloff_point)
}

/// Root Mean Square (RMS)
///
/// Represents the quadratic mean of the waveform, and is indicative of its perceived loudness.
/// This concept is analogous to the 'Energy' feature in YAAFE, and is inspired by Loy's "Musimathics".
///
/// RMS provides insights into the sound's perceived volume or intensity.
///
/// # Parameters
///
/// * `signal` - A reference to a vector of the signal samples.
/// # Returns
///
/// A floating point value representing the RMS of the signal.
///
pub fn get_rms(signal: &[f64]) -> f64 {
    features::spectral::rms::compute(signal)
}

/// Zero Crossing Rate
///
/// Represents the count of zero crossings in the provided data segment.
///
/// Useful in distinguishing between rhythmic and tonal sounds. While rhythmic or percussive
/// audio typically exhibits variable zero-crossing rates across segments, tonal sounds
/// tend to display a steadier rate.
///
/// # Parameters
///
/// * `signal` - A slice of the signal samples.
///
/// # Returns
///
/// A floating point value representing the ZCR of the signal.
///
pub fn get_zcr(signal: &[f64]) -> f64 {
    features::spectral::zcr::compute(signal)
}
