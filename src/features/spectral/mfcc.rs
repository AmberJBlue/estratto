use crate::features::spectral::power_spectrum;
use crate::filters::mel::compute;

pub fn mfcc(signal: &[f64], num_cepstrals: usize) -> Vec<f64> {
    let fft_size = 512;
    let sample_rate = 44100;
    let num_filters = 40; // Number of Mel filters
    let power_spec = power_spectrum(signal);
    let filter_bank = mel_filter_bank(num_filters, fft_size, sample_rate);
    let mel_energies = filter_bank.dot(&power_spec);
    let log_mel_energies = mel_energies.mapv(f64::ln);

    // Perform DCT
    let mfccs = dct(&log_mel_energies)
        .slice(s![0..num_cepstrals])
        .to_owned();

    // Return MFCCs as Vec<f64>
    mfccs
        .iter()
        .map(|&x| x)
        .collect()
}
