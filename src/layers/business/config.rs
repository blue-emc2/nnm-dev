// Configuration business logic
use crate::layers::data::repositories::config_repository::ConfigRepository;
use crate::layers::data::repositories::history_repository::HistoryRepository;
use crate::models::config::{Config, ConfigMessage};
use crate::models::errors::AppError;
use crate::app::file::File;

pub struct ConfigBusinessLayer {
    config_repo: ConfigRepository,
    history_repo: HistoryRepository,
}

impl ConfigBusinessLayer {
    pub fn new() -> Self {
        Self {
            config_repo: ConfigRepository::new(),
            history_repo: HistoryRepository::new(),
        }
    }

    /// 初期設定を作成
    pub fn create(&self) -> Result<ConfigMessage, AppError> {
        let config = Config::new();
        let config_file_path = config.file_path();

        if config_file_path.exists() {
            return Ok(ConfigMessage::ExistsConfig);
        }

        self.config_repo.save(&config)?;

        // 履歴ファイルも作成
        let history = crate::models::history::History::new();
        self.history_repo.save(&history)?;

        Ok(ConfigMessage::Success(
            config_file_path.into_os_string().into_string().unwrap(),
        ))
    }
}
