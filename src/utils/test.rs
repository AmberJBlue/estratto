#[cfg(test)]
pub mod data {
    extern crate serde;
    extern crate serde_json;

    use std::f64;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    pub struct TestDataInfo {
        pub description: String,
        pub sampleRate: i32,
        pub bufferSize: i32,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    pub struct TestDataFeatures {
        pub energy: f64,
        pub loudness: Vec<f64>,
        pub mfcc: Vec<f64>,
        pub perceptualSharpness: f64,
        pub perceptualSpread: f64,
        pub powerSpectrum: Vec<f64>,
        pub rms: f64,
        pub spectralCentroid: f64,
        pub spectralFlatness: f64,
        pub spectralKurtosis: f64,
        pub spectralRolloff: f64,
        pub spectralSkewness: f64,
        pub spectralSlope: f64,
        pub spectralSpread: f64,
        pub amplitudeSpectrum: Vec<f64>,
        pub zcr: f64,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    pub struct TestDataSet {
        pub info: TestDataInfo,
        pub signal: Vec<f64>,
        pub features: TestDataFeatures,
    }

    pub fn load_dataset(dataset_path: &str) -> TestDataSet {
        let path = Path::new(dataset_path);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut json_str = String::new();
        match file.read_to_string(&mut json_str) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => (),
        }

        let data: TestDataSet = match serde_json::from_str(&json_str) {
            Ok(d) => d,
            Err(err) => panic!("{:?}", err),
        };

        data
    }

    pub fn get_all() -> Vec<TestDataSet> {
        let paths = ["./src/utils/data.json"];

        paths
            .iter()
            .map(|p| load_dataset(p))
            .collect()
    }
}
