use crate::{layers::data::repositories::history_repository::HistoryRepository, models::history::History};
use crate::models::errors::AppError;

pub struct HistoryBusinessLayer {
    history_repo: HistoryRepository,
}

impl HistoryBusinessLayer {
    pub fn new() -> Self {
        Self {
            history_repo: HistoryRepository::new(),
        }
    }

    pub fn find_or_create_history(&self) -> Result<History, AppError> {
        if self.history_repo.history_exists()? {
            self.load()
        } else {
            self.history_repo.create()
        }
    }

    pub fn load(&self) -> Result<History, AppError> {
        self.history_repo.load()
    }
}
