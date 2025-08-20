pub mod config;
pub mod controller;
pub mod prompt;
pub mod error;

mod entity;
mod file;
mod history;
mod parser;
mod screen;
mod table;

use std::collections::HashMap;

use controller::{
    bookmark_controller::BookmarkController, config_controller::ConfigController,
    history_controller::HistoryController, rss_controller::RssController,
};

pub struct App {
    pub rss: RssController,
    pub bookmark: BookmarkController,
    pub history: HistoryController,
    pub config: ConfigController,
}

impl App {
    pub fn new() -> Self {
        let app = App {
            rss: RssController::default(),
            bookmark: BookmarkController,
            history: HistoryController,
            config: ConfigController,
        };
        app
    }

    pub fn fetch_articles(&mut self, options: HashMap<String, String>) {
        self.rss.index(options);
    }
}
