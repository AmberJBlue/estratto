#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate approx;

mod features;
mod utils;

pub type Hz = utils::Hz;

pub fn get_zcr(signal: &[f64]) -> f64 {
    features::zcr::compute(signal)
}

pub fn get_rms(signal: &[f64]) -> f64 {
    features::rms::compute(signal)
}

pub fn get_amplitude_spectrum(signal: &[f64]) -> Vec<f64> {
    features::amplitude_spectrum::compute(signal)
}
