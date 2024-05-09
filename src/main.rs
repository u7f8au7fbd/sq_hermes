#[macro_use]
mod macros;
mod config;
mod html;
mod json;
mod tui;

use serde::{Deserialize, Serialize};
use std::process::Command;
#[derive(Debug, Deserialize, Serialize, Clone)] // Add this line
pub struct Config {
    api_key: String,
    search_engine_id: String,
    world: [String; 10],
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd!(utf - 8);
    cmd!(line);
    let mut config: Vec<Config> = vec![Config {
        api_key: "NULL".to_string(),
        search_engine_id: "NULL".to_string(),
        world: core::array::from_fn(|_| "NULL".to_string()),
    }];
    read_cofig(config.clone());
    config::load(&mut config);
    read_cofig(config);
    //tui::sub()?;
    html::sub();
    json::sub();
    Ok(())
}

fn read_cofig(config: Vec<Config>) {
    println!("{:#?}", config)
}
