use crate::app::prompt::Prompt;
use crate::commands::Actions;
use crate::layers::business::rss::RssBusinessLayer;
use crate::layers::presentation::cli::tui::{self, KeyAction};
use crate::models::errors::AppError;
use ratatui::widgets::ListState;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::mpsc;
use std::thread;

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
                let (tx, rx) = mpsc::channel();
                let mut articles = vec![];
                let mut message = "取得中...".to_string();
                thread::scope(|s| {
                    s.spawn(move || {
                        let results = self.rss_business.fetch_articles();
                        tx.send(results).unwrap();
                    });

                    ratatui::run(|terminal| -> Result<(), AppError> {
                        let mut state = ListState::default().with_selected(Some(0));

                        loop {
                            match rx.try_recv() {
                                Ok(Ok(received)) => {
                                    articles = received;
                                    message = if articles.is_empty() {
                                        "新着記事はありませんでした".to_string()
                                    } else {
                                        format!("{}件の新着", articles.len())
                                    };
                                }
                                Ok(Err(AppError::FileError(e)))
                                    if e.kind() == ErrorKind::NotFound =>
                                {
                                    message = "設定ファイルが見つかりませんでした。nnm init で初期設定を行ってください。".to_string();
                                }
                                Ok(Err(e)) => {
                                    message = format!("エラーが発生しました。{}", e);
                                }
                                Err(_) => (),
                            };

                            terminal.draw(|frame| {
                                tui::render(frame, &articles, message.as_str(), &mut state)
                            })?;
                            match tui::read_key_action()? {
                                KeyAction::Quit => return Ok(()),
                                KeyAction::Up => state.select_previous(),
                                KeyAction::Down => state.select_next(),
                                KeyAction::Nothing => (),
                            }
                        }
                    })
                })?;
            }
        }
        Ok(())
    }
}
