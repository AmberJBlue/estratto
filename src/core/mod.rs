pub fn fftfreq(n: usize, sample_rate: usize) -> Vec<f64> {
    let mut freqs = Vec::new();
    let sr: f64 = sample_rate as f64;
    let factor = 1.0 / sr;
    for i in 0..=n / 2 {
        freqs.push((i as f64) * factor);
    }
    freqs
}
