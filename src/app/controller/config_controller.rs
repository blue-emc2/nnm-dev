use std::{env, path::PathBuf};

use crate::app::{
    config::{Config, ConfigMessage},
    history::History,
};

pub struct ConfigController;

impl ConfigController {
    pub fn create(&self) -> Result<ConfigMessage, std::io::Error> {
        let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut config_dir = PathBuf::from(home_dir);
        config_dir.push(".config/nnm");

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }

        let config_file_path = config_dir.join("config.json");
        if config_file_path.exists() {
            return Ok(ConfigMessage::ExistsConfig);
        }
        let config = Config::new();
        config.save()?;

        let history = History::new();
        history.save()?;

        Ok(ConfigMessage::Success(
            config_file_path.into_os_string().into_string().unwrap(),
        ))
    }
}
