# estratto - An Audio Feature Extraction Library

![estratto Logo](logo-banner.png)

[![build](https://github.com/AmberJBlue/estratto/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/AmberJBlue/estratto/actions/workflows/build.yml)
[![all tests](https://github.com/AmberJBlue/estratto/actions/workflows/test.yml/badge.svg)](https://github.com/AmberJBlue/estratto/actions/workflows/test.yml)
[![linter](https://github.com/AmberJBlue/estratto/actions/workflows/lint.yml/badge.svg)](https://github.com/AmberJBlue/estratto/actions/workflows/lint.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](link-to-your-documentation-if-any)

**estratto** is a powerful and user-friendly Rust library designed for extracting rich audio features from digital audio signals. Whether you're analyzing music, speech, or any other type of audio data, estratto provides an array of feature extraction techniques that help you uncover valuable insights and patterns.

## 🎵 Features

estratto equips developers with a comprehensive suite of audio feature extraction tools:

### Spectral

- **Amplitude Spectrum**: Extract the magnitude spectrum of an audio signal, showcasing its frequency components and their respective intensities.

- **Power Spectrum**: Dive deeper into the frequency domain by squaring the amplitude spectrum.

- **Spectral Centroid**: An intuitive measure for understanding the "brightness" or tonal characteristic of a sound.

- **Spectral Rolloff**: Determine the threshold frequency, below which a set percentage of the spectrum's energy is contained.

- **RMS (Root Mean Square)**: Understand the perceived loudness or intensity of your sound using this quadratic mean of the waveform.

- **Zero Crossing Rate**: Differentiate rhythmic sounds from tonal ones with this feature.

- **...and more**: This is still a work in progress stay tuned for more features!

## 🚀 Getting Started

### Installation

Simply add estratto to your Rust project's `Cargo.toml`:

```toml
[dependencies]
estratto = "0.1.0"
```

### Run Tests

```sh
cargo test
```

## 📜 License

estratto is open-sourced under the [MIT License](https://opensource.org/licenses/MIT).

### Running Tests

```sh
cargo test
```
