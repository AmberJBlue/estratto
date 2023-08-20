/// Computes the autocorrelation of an audio signal.
///
/// # Arguments
///
/// * `samples` - A slice of audio samples.
/// * `buffer_size` - The size of the autocorrelation buffer.
///
/// # Returns
///
/// A vector containing the autocorrelation values.
pub fn autocorrelate(samples: &[f32], buffer_size: usize) -> Vec<f32> {
    let mut autocorrelation = vec![0.0; buffer_size];
    for i in 0..buffer_size {
        for j in 0..samples.len() - i {
            autocorrelation[i] += samples[j] * samples[j + i];
        }
    }
    autocorrelation
}