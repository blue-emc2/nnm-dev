use std::path::PathBuf;
use crate::models::config::Config;
use crate::models::errors::AppError;

pub struct ConfigRepository;

impl ConfigRepository {
    pub fn new() -> Self {
        ConfigRepository
    }

    pub fn load(&self) -> Result<Config, AppError> {
        Config::load().map_err(AppError::from)
    }

    pub fn save(&self, config: &Config) -> Result<(), AppError> {
        config.save().map_err(AppError::from)
    }

    pub fn get_default_path(&self) -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nnm")
            .join("config.json")
    }
}
