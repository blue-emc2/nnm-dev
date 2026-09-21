use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::file::File;

const DEFAULT_DISPLAY_LIMIT: i32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    display_limit: i32,
    chunk_size: i32,
    bookmarks: Vec<String>,  // お気に入り一覧
    links: Vec<String>,      // rssのリンク一覧
    history_expiration: i32, // 履歴の保持期間(日)
}

pub enum ConfigMessage {
    ExistsConfig,
    Success(String),
    // Error(String), 後で実装する
}

impl File for Config {
    fn file_path(&self) -> PathBuf {
        // XDG_CONFIG_HOMEが設定されていれば、そこが設定パス
        // 未設定の場合は、~/.configが変える
        let root_config_path = match std::env::var("XDG_CONFIG_HOME") {
            Ok(val) => Some(PathBuf::from(val)),
            Err(_) => std::env::home_dir().map(|home| home.join(".config")),
        };
        root_config_path
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nnm")
            .join("config.json")
    }
}

#[allow(dead_code)]
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

    pub fn save(&self) -> Result<(), std::io::Error> {
        self.save_to_file(self)?;
        Ok(())
    }

    pub fn load() -> Result<Self, std::io::Error> {
        let config = Config::new();
        config.load_from_file()
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

    // クロスプラットフォーム対応のconfig配置
    pub fn default_file_path(&self) -> PathBuf {
        self.file_path()
    }
}
