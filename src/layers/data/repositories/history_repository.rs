use std::io::Write;
use std::{fs, fs::File};
use std::path::PathBuf;
use crate::app::file::app_config_dir;
use crate::models::history::{self, History};
use crate::models::errors::AppError;
use chrono::Local;

pub struct HistoryRepository;

impl HistoryRepository {
    pub fn new() -> Self {
        HistoryRepository
    }

    pub fn history_exists(&self) -> Result<bool, std::io::Error> {
        let path = self.get_history_file_path();
        return fs::exists(path)
    }

    pub fn create(&self) -> Result<History, AppError> {
        let history = History::new();
        self.save(&history)?;
        Ok(history)
    }

    pub fn load(&self) -> Result<History, AppError> {
        let path = self.get_history_file_path();
        let config = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        Ok(config)
    }

    pub fn save(&self, history: &History) -> Result<(), AppError> {
        let history_file_path = self.get_history_file_path();

        // ディレクトリが存在しない場合は作成
        if let Some(parent) = history_file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_json = serde_json::to_string_pretty(&history)?;
        let mut file = File::create(history_file_path)?;
        write!(file, "{}", config_json)?;
        Ok(())
    }

    fn get_history_file_path(&self) -> PathBuf {
        let file_name = Local::now().format("%Y%m%d-articles.json").to_string();
        return app_config_dir().join("history").join(file_name)
    }
}
