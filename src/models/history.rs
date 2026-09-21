use crate::app::file::File;
use crate::models::article::Article;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    last_fetched_date: String,
    entity: Vec<Article>,
}

impl File for History {
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
            .join("history.json")
    }
}

impl History {
    pub fn new() -> Self {
        History {
            last_fetched_date: "".to_string(),
            entity: Vec::new(),
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        self.save_to_file(self)?;
        Ok(())
    }

    pub fn load() -> Result<Self, std::io::Error> {
        History::new().load_from_file()
    }

    pub fn update_last_fetched_date(&mut self) {
        let now = Local::now();
        self.last_fetched_date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    pub fn entity_push(&mut self, entity: Article) {
        self.entity.push(entity);
    }

    // TODO: iteratorを実装したほうかいい？いちいちコピーしたものを返すのは無駄かも
    pub fn get_entities(&self) -> Vec<Article> {
        self.entity.clone()
    }
}
