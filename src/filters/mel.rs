use ndarray::{ Array, Array1, Array2 };
use crate::core::hz_to_mel;
use crate::core::mel_to_hz;

pub fn compute(num_filters: usize, fft_size: usize, sample_rate: usize) -> Array2<f64> {
    let mut filter_bank = Array2::zeros((num_filters, fft_size / 2));
    let min_hz = 0.0;
    let max_mel = hz_to_mel((sample_rate as f64) / 2.0);
    let mel_points: Array1<f64> = Array::linspace(hz_to_mel(min_hz), max_mel, num_filters + 2);
    let bin_points: Array1<usize> = mel_points.map(
        |mel| (mel_to_hz(*mel) / ((sample_rate as f64) / (fft_size as f64))).round() as usize
    );

    for i in 0..num_filters {
        for f in bin_points[i]..bin_points[i + 1] {
            filter_bank[[i, f]] =
                ((f - bin_points[i]) as f64) / ((bin_points[i + 1] - bin_points[i]) as f64);
        }
        for f in bin_points[i + 1]..bin_points[i + 2] {
            filter_bank[[i, f]] =
                1.0 -
                ((f - bin_points[i + 1]) as f64) / ((bin_points[i + 2] - bin_points[i + 1]) as f64);
        }
    }
    filter_bank
}
