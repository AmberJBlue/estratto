# Estratto - An Audio Feature Extraction Library

[![Build Status](https://travis-ci.org/your-username/audio-feature-extraction.svg?branch=main)](https://travis-ci.org/your-username/audio-feature-extraction)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Estratto is a Rust library provides various audio feature extraction techniques for processing and analyzing audio signals. It is designed to be modular, easy to use, and extensible for different audio analysis tasks.

## Features

- MFCC (Mel-Frequency Cepstral Coefficients) feature extraction
- Pitch detection algorithms
- (Add more features here)

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