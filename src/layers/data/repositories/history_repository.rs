use std::path::PathBuf;
use crate::models::history::History;
use crate::models::errors::AppError;

pub struct HistoryRepository;

impl HistoryRepository {
    pub fn new() -> Self {
        HistoryRepository
    }

    pub fn load(&self) -> Result<History, AppError> {
        History::load().map_err(AppError::from)
    }

    pub fn save(&self, history: &History) -> Result<(), AppError> {
        history.save().map_err(AppError::from)
    }

    pub fn get_default_path(&self) -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nnm")
            .join("history.json")
    }
}
