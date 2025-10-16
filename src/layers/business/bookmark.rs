// Bookmark business logic
use crate::layers::data::repositories::config_repository::ConfigRepository;
use crate::models::errors::AppError;

pub struct BookmarkBusinessLayer {
    config_repo: ConfigRepository,
}

impl BookmarkBusinessLayer {
    pub fn new() -> Self {
        Self {
            config_repo: ConfigRepository::new(),
        }
    }

    /// ブックマークを追加
    pub fn add_bookmark(&self, url: &str) -> Result<String, AppError> {
        let mut config = self.config_repo.load()?;
        let bookmarks = config.bookmarks();

        if bookmarks.contains(&url.to_string()) {
            return Ok(url.to_string());
        }

        let bookmarks = config.mut_bookmarks();
        bookmarks.push(url.to_string());
        self.config_repo.save(&config)?;
        Ok(url.to_string())
    }

    /// ブックマークを削除
    pub fn delete_bookmark(&self, url: &str) -> Result<(), AppError> {
        let mut config = self.config_repo.load()?;
        let bookmarks = config.mut_bookmarks();

        let index = bookmarks
            .iter()
            .position(|x| x == url)
            .ok_or_else(|| AppError::ParseError("URLが見つかりません".to_string()))?;

        bookmarks.remove(index);
        self.config_repo.save(&config)?;
        Ok(())
    }

    /// ブックマーク一覧を取得
    pub fn list_bookmarks(&self) -> Result<Vec<String>, AppError> {
        let config = self.config_repo.load()?;
        Ok(config.bookmarks().clone())
    }
}
