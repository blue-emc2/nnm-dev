use crate::models::article::Article;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::{List, ListState};

pub enum KeyAction {
    Up,
    Down,
    Quit,
    Nothing,
}

pub fn render(frame: &mut ratatui::Frame, articles: &[Article], state: &mut ListState) {
    let titles = articles.iter().map(|a| a.title.as_str());
    let list = List::new(titles).highlight_symbol("> ");
    frame.render_stateful_widget(list, frame.area(), state);
}

pub fn read_key_action() -> std::io::Result<KeyAction> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => Ok(KeyAction::Quit),
            KeyCode::Char('j') | KeyCode::Down => Ok(KeyAction::Down),
            KeyCode::Char('k') | KeyCode::Up => Ok(KeyAction::Up),
            _ => Ok(KeyAction::Nothing),
        },
        _ => Ok(KeyAction::Nothing),
    }
}
