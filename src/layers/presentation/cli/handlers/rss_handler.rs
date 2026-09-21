use crate::app::prompt::Prompt;
use crate::commands::Actions;
use crate::layers::business::rss::RssBusinessLayer;
use crate::models::errors::AppError;
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
                match self.rss_business.fetch_articles() {
                    Ok(articles) => {
                        if articles.is_empty() {
                            println!("新しい記事はありません。");
                        } else {
                            // Display層に委譲
                            use crate::layers::presentation::cli::display::Display;
                            let display = Display::new();
                            display.draw_articles(&articles, options);
                        }
                    }
                    Err(AppError::FileError(e)) if e.kind() == ErrorKind::NotFound => {
                        eprintln!("設定ファイルが見つかりませんでした。\nnnm init で初期設定を行ってください。");
                    }
                    Err(e) => {
                        eprintln!("エラーが発生しました。\n{}", e);
                    }
                }
            }
        }
        Ok(())
    }
}
