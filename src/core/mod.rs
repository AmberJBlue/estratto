pub type Hz = f64;

pub fn hz_to_mel(hz: f64) -> f64 {
    2595.0 * (hz / 700.0 + 1.0).log10()
}

pub fn mel_to_hz(mel: f64) -> f64 {
    700.0 * ((10.0f64).powf(mel / 2595.0) - 1.0)
}

pub fn mu(exp: i32, vector: &[f64]) -> f64 {
    let fraction = vector
        .iter()
        .enumerate()
        .fold((0.0, 0.0), |acc, x| {
            let num_inc = (x.0 as f64).powi(exp) * x.1.abs();
            let den_inc = x.1;

            (acc.0 + num_inc, acc.1 + den_inc)
        });

    fraction.0 / fraction.1
}
