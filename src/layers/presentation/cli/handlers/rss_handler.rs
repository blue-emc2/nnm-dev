use crate::app::prompt::Prompt;
use crate::commands::Actions;
use crate::layers::business::rss::RssBusinessLayer;
use crate::layers::presentation::cli::tui::{self, KeyAction};
use crate::models::errors::AppError;
use ratatui::widgets::ListState;
use std::collections::HashMap;
use std::io::ErrorKind;

pub struct RssHandler {
    rss_business: RssBusinessLayer,
}

impl Prompt for RssHandler {
    fn exec_delete_link(&self, url: &str) {
        match self.rss_business.delete_feed(url) {
            Ok(_) => println!("{}を削除しました", url),
            Err(e) => eprintln!("削除エラー: {}", e),
        }
    }
}

impl RssHandler {
    pub fn new() -> Self {
        Self {
            rss_business: RssBusinessLayer::new(),
        }
    }

    pub fn handle_rss_command(
        &self,
        action: Option<Actions>,
        options: HashMap<String, String>,
    ) -> Result<(), AppError> {
        match action {
            Some(Actions::Add { url }) => {
                if let Some(url) = url {
                    let result = self.rss_business.add_feed(&url)?;
                    println!("{} を追加しました", result);
                }
            }
            Some(Actions::Delete) => {
                let feeds = self.rss_business.list_feeds()?;
                let mut feeds_mut = feeds.clone();
                self.delete_prompt(&mut feeds_mut);
            }
            None => {
                // 記事の取得と表示
                let articles = match self.rss_business.fetch_articles() {
                    Ok(articles) => articles,
                    Err(AppError::FileError(e)) if e.kind() == ErrorKind::NotFound => {
                        eprintln!("設定ファイルが見つかりませんでした。\nnnm init で初期設定を行ってください。");
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("エラーが発生しました。\n{}", e);
                        return Ok(());
                    }
                };

                ratatui::run(|terminal| -> Result<(), AppError> {
                    let mut state = ListState::default().with_selected(Some(0));
                    loop {
                        terminal.draw(|frame| tui::render(frame, &articles, &mut state))?;
                        match tui::read_key_action()? {
                            KeyAction::Quit => return Ok(()),
                            KeyAction::Up => state.select_previous(),
                            KeyAction::Down => state.select_next(),
                            KeyAction::Nothing => (),
                        }
                    }
                })?;
            }
        }
        Ok(())
    }
}
