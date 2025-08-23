use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::file::File;

const DEFAULT_DISPLAY_LIMIT: i32 = 10;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    display_limit: i32,
    chunk_size: i32,
    bookmarks: Vec<String>,   // お気に入り一覧
    links: Vec<String>,       // rssのリンク一覧
    history_expiration: i32, // 履歴の保持期間(日)
}

pub enum ConfigMessage {
    ExistsConfig,
    Success(String),
    // Error(String), 後で実装する
}

impl File for Config {
    fn file_path(&self) -> PathBuf {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut config_dir = PathBuf::from(home_dir);
        config_dir.push(".config/nnm/config.json");
        config_dir
    }
}

impl Config {
    pub fn new() -> Self {
        Config {
            chunk_size: 10,
            bookmarks: Vec::new(),
            links: Vec::new(),
            display_limit: DEFAULT_DISPLAY_LIMIT,
            history_expiration: 90,
        }
    }

    pub fn links(&self) -> &Vec<String> {
        self.links.as_ref()
    }

    pub fn mut_links(&mut self) -> &mut Vec<String> {
        &mut self.links
    }

    pub fn bookmarks(&self) -> &Vec<String> {
        self.bookmarks.as_ref()
    }

    pub fn mut_bookmarks(&mut self) -> &mut Vec<String> {
        &mut self.bookmarks
    }

    pub fn chunk_size(&self) -> i32 {
        self.chunk_size
    }

    // 今はhome下にしか作れない
    pub fn default_file_path(&self) -> PathBuf {
        self.file_path()
    }
}
