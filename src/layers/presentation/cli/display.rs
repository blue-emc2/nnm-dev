// Display and output modules
use crate::models::article::Article;
use crate::app::table::row::Row;
use crate::app::table::table::Table;
use std::collections::HashMap;

pub struct Display;

impl Display {
    pub fn new() -> Self {
        Display
    }

    pub fn draw_articles(&self, articles: &[Article], options: HashMap<String, String>) {
        let (width, height) = crossterm::terminal::size().unwrap_or_else(|_| (80, 24));
        let mut table = Table::new();
        let header = Row::from(vec!["No".to_string(), "Body".to_string()]);
        table
            .set_size(width, height)
            .set_header(header)
            .set_draw_options(options);

        for article in articles.iter() {
            let title = article.title.clone();
            let description = article.description.clone();
            let link = article.link.clone();

            let row = Row::from(vec![title, description, link]);
            table.add_row(row);
        }

        println!("{}", table);
    }

    pub fn display_message(&self, message: &str) {
        println!("{}", message);
    }
}
