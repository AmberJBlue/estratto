#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate approx;

mod features;
mod utils;

pub type Hz = utils::Hz;

pub fn get_zcr(signal: &Vec<f64>) -> f64 {
    features::zcr::compute(signal)
}
