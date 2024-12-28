use serde::{Deserialize, Serialize};
use std::fs;
use crate::error::BuildError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_url: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    pub target_platform: String,
    pub build_mode: String,
    pub extra_flags: Vec<String>,
}

pub fn save_config(server: &str, api_key: &str) -> Result<(), BuildError> {
    let config = ServerConfig {
        server_url: server.to_string(),
        api_key: api_key.to_string(),
    };
    
    let config_dir = dirs::config_dir()
        .ok_or_else(|| BuildError::ConfigError("Could not find config directory".into()))?
        .join("flutter-cross-builder");
    
    fs::create_dir_all(&config_dir)?;
    
    let config_path = config_dir.join("config.json");
    let config_json = serde_json::to_string_pretty(&config)?;
    
    fs::write(config_path, config_json)?;
    
    Ok(())
}
