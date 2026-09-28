use crate::models::article::Article;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::Layout;
use ratatui::style::Stylize;
use ratatui::widgets::{List, ListState, Paragraph};
use std::time::Duration;

pub enum KeyAction {
    Up,
    Down,
    Quit,
    Nothing,
}

pub fn render(
    frame: &mut ratatui::Frame,
    articles: &[Article],
    status_line: &str,
    state: &mut ListState,
) {
    let titles = articles.iter().map(|a| a.title.as_str());
    let list = List::new(titles).highlight_symbol("> ");
    let vertical = Layout::vertical([Fill(1), Length(1)]);
    let [main_area, status_area] = vertical.areas(frame.area());
    let paragraph = Paragraph::new(status_line).red().on_white().bold();
    frame.render_stateful_widget(list, main_area, state);
    frame.render_widget(paragraph, status_area);
}

pub fn read_key_action() -> std::io::Result<KeyAction> {
    if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => Ok(KeyAction::Quit),
                KeyCode::Char('j') | KeyCode::Down => Ok(KeyAction::Down),
                KeyCode::Char('k') | KeyCode::Up => Ok(KeyAction::Up),
                _ => Ok(KeyAction::Nothing),
            },
            _ => Ok(KeyAction::Nothing),
        }
    } else {
        Ok(KeyAction::Nothing)
    }
}
