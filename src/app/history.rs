use super::file::File;
use crate::app::entity::Entity;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    last_fetched_date: String,
    entity: Vec<Entity>,
}

impl File for History {
    fn file_path(&self) -> PathBuf {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut config_dir = PathBuf::from(home_dir);
        config_dir.push(".config/nnm/history.json");
        config_dir
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

    pub fn update_last_fetched_date(&mut self) {
        let now = Local::now();
        self.last_fetched_date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    pub fn entity_push(&mut self, entity: Entity) {
        self.entity.push(entity);
    }

    // TODO: iteratorを実装したほうかいい？いちいちコピーしたものを返すのは無駄かも
    pub fn get_entities(&self) -> Vec<Entity> {
        self.entity.clone()
    }
}
