use crate::models::article::Article;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::List;

pub fn render(frame: &mut ratatui::Frame, articles: &[Article]) {
    let titles = articles.iter().map(|a| a.title.as_str());
    let list = List::new(titles);
    frame.render_widget(list, frame.area());
}

pub fn should_quit() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}
