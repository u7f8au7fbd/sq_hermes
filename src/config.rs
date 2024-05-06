use serde::*;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)] // Add this line
pub struct Config {
    api_key: String,
    search_engine_id: String,
    world: [String; 10],
}

pub fn write() {
    let filename = "config.json";
    let api_key = "API";
    let search_engine_id = "SEI";
    let world0 = [
        "world0".to_string(),
        "world1".to_string(),
        "world2".to_string(),
        "world3".to_string(),
        "world4".to_string(),
        "world5".to_string(),
        "world6".to_string(),
        "world7".to_string(),
        "world8".to_string(),
        "world9".to_string(),
    ];

    let world1 = [
        "world101".to_string(),
        "world11".to_string(),
        "world12".to_string(),
        "world13".to_string(),
        "world14".to_string(),
        "world15".to_string(),
        "world16".to_string(),
        "world17".to_string(),
        "world18".to_string(),
        "world19".to_string(),
    ];

    let world2 = [
        "world20".to_string(),
        "world21".to_string(),
        "world22".to_string(),
        "world23".to_string(),
        "world24".to_string(),
        "world25".to_string(),
        "world26".to_string(),
        "world27".to_string(),
        "world128".to_string(),
        "world29".to_string(),
    ];

    let world3 = [
        "world30".to_string(),
        "world31".to_string(),
        "world32".to_string(),
        "world33".to_string(),
        "world34".to_string(),
        "world35".to_string(),
        "world36".to_string(),
        "world37".to_string(),
        "world38".to_string(),
        "world39".to_string(),
    ];

    let configs = [
        Config {
            api_key: api_key.to_string(),
            search_engine_id: search_engine_id.to_string(),
            world: world0,
        },
        Config {
            api_key: api_key.to_string(),
            search_engine_id: search_engine_id.to_string(),
            world: world1,
        },
        Config {
            api_key: api_key.to_string(),
            search_engine_id: search_engine_id.to_string(),
            world: world2,
        },
        Config {
            api_key: api_key.to_string(),
            search_engine_id: search_engine_id.to_string(),
            world: world3,
        },
    ];

    let json = serde_json::to_string_pretty(&configs).unwrap();
    let mut file = File::create(filename).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn read() {
    let filename = "config.json";
    let file = File::open(filename).unwrap();
    let configs: Vec<Config> = serde_json::from_reader(file).unwrap();
    for config in configs {
        println!("{:#?}", config);
    }
}
