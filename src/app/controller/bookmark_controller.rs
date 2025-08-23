use std::io;

use crate::app::{config::Config, file::File, prompt::Prompt};

pub struct BookmarkController;

impl Prompt for BookmarkController {
    fn exec_delete_link(&self, url: &str) {
    let mut config: Config = match Config::new().load_from_file() {
            Ok(c) => c,
            Err(e) => { eprintln!("設定読み込みエラー: {}", e); return; }
        };
    let links = config.mut_bookmarks();
    let index = match links.iter().position(|x| x == url) {
            Some(i) => i,
            None => { eprintln!("URLが見つかりません"); return; }
        };
    links.remove(index);
    if let Err(e) = config.save_to_file(config.clone()) {
            eprintln!("設定保存エラー: {}", e);
        }
    }
}

impl BookmarkController {
    pub fn add_link(&self, url: &str) -> Result<String, io::Error> {
        let mut config: Config = Config::new().load_from_file()?;
        let bookmarks = config.bookmarks();

        if bookmarks.contains(&url.to_string()) {
            return Ok(url.to_string());
        }
        let bookmarks = config.mut_bookmarks();
        bookmarks.push(url.to_string());
        config.save_to_file(config.clone())?;
        Ok(url.to_string())
    }

    pub fn delete_link(&self) -> Result<(), io::Error> {
        let mut config: Config = Config::new().load_from_file()?;
        self.delete_prompt(config.mut_bookmarks());
        Ok(())
    }

    pub fn show(&self) -> Result<(), io::Error> {
        let config: Config = Config::new().load_from_file()?;
        for link in config.bookmarks() {
            println!("{}", link);
        }
        Ok(())
    }
}
