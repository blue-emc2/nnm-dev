use std::collections::HashMap;

use crate::app::{history::History, screen};

pub struct HistoryController;

impl HistoryController {
    pub fn show(&self) {
        let screen = screen::Screen::new();
        let history = History::load();
        match history {
            Ok(history) => {
                let entities = history.get_entities();
                screen.draw(&entities, HashMap::new());
            }
            Err(e) => {
                eprintln!("Error loading history: {:?}", e);
            }
        }
    }
}
