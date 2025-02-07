use std::fs;
use serde::Deserialize;
use log::info;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub ble: BLEConfig,
    pub audio: AudioConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub endpoint: String,
}

#[derive(Deserialize)]
pub struct BLEConfig {
    pub scan_interval: u32,
}

#[derive(Deserialize)]
pub struct AudioConfig {
    pub sample_rate: u32,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    info!("Configuration loaded successfully");
    Ok(config)
}
