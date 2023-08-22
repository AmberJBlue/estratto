# estratto - An Audio Feature Extraction Library

![estratto Logo](logo-banner.png)

![example workflow](https://github.com/github/docs/actions/workflows/main.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](link-to-your-documentation-if-any)

**estratto** is a powerful and user-friendly Rust library designed for extracting rich audio features from digital audio signals. Whether you're analyzing music, speech, or any other type of audio data, estratto provides an array of feature extraction techniques that help you uncover valuable insights and patterns.

## 🎵 Features

estratto equips developers with a comprehensive suite of audio feature extraction tools:

- **Amplitude Spectrum**: Extract the magnitude spectrum of an audio signal, showcasing its frequency components and their respective intensities. Use `get_amplitude_spectrum` for a vector representation of the amplitude.

- **Power Spectrum**: Dive deeper into the frequency domain by squaring the amplitude spectrum. Fetch it using `get_power_spectrum`.

- **Spectral Centroid**: An intuitive measure for understanding the "brightness" or tonal characteristic of a sound. Fetch it using `get_spectral_centroid`.

- **Spectral Rolloff**: Determine the threshold frequency, below which a set percentage of the spectrum's energy is contained. Extract it with `get_spectral_rolloff`.

- **RMS (Root Mean Square)**: Understand the perceived loudness or intensity of your sound using this quadratic mean of the waveform. Use `get_rms` to get insights.

- **Zero Crossing Rate**: Differentiate rhythmic sounds from tonal ones with this feature. Use `get_zcr` for the exact rate.

- **...and more**: This is still a work in progress stay tuned for more features!


## 🚀 Getting Started

### Installation

Simply add estratto to your Rust project's `Cargo.toml`:

```toml
[dependencies]
estratto = "0.1.0"
```

### Documentation

Dive deeper into estratto's capabilities with our [comprehensive documentation](link-to-your-documentation).


### Run Tests

```sh
cargo test
```

## 📜 License

estratto is open-sourced under the [MIT License](https://opensource.org/licenses/MIT).

---

Remember to replace placeholders (like `link-to-your-logo-if-any`) with the actual links or content. This template assumes some level of community interaction, so consider what sections make sense for your project as it stands and as you envision it growing in the future.
## Features

- MFCC (Mel-Frequency Cepstral Coefficients) feature extraction
- Pitch detection algorithms
- (Add more features here... coming soon)

## Getting Started
To use this library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
estratto = "0.1.0"
```

### Usage
Below is an example of how you can use the library to extract MFCC features from an audio signal:

```rust
use audio_feature_extraction::mfcc::extract_mfcc;

fn main() {
    // Load audio signal (replace this with actual audio loading code)
    let audio_signal = vec![/* audio samples */];

    // Extract MFCC features
    let mfcc_features = extract_mfcc(&audio_signal);

    // Process the extracted features as needed
    // (e.g., store, analyze, visualize, etc.)
}
```

### Running Tests

```sh
cargo test
```
