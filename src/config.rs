use crate::Config;
use std::fs::File;

pub fn load(config: &mut Vec<Config>) {
    let filename = "config.json";
    let file = File::open(filename).unwrap();
    let load_configs: Vec<Config> = serde_json::from_reader(file).unwrap();
    *config = load_configs;
}
