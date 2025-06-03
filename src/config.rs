use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub projects_path: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap();
    let final_config_path = home_dir.join(".config/stevio_code/config.toml");
    let content = fs::read_to_string(final_config_path).expect("Could not read config file.");
    let config: Config = toml::from_str(&content).expect("Could not parse config file.");
    Ok(config)
}
