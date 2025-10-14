use crate::layers::business::bookmark::BookmarkBusinessLayer;
use crate::models::errors::AppError;
use crate::commands::Actions;
use crate::app::prompt::Prompt;

pub struct BookmarkHandler {
    bookmark_business: BookmarkBusinessLayer,
}

impl Prompt for BookmarkHandler {
    fn exec_delete_link(&self, url: &str) {
        match self.bookmark_business.delete_bookmark(url) {
            Ok(_) => println!("{}を削除しました", url),
            Err(e) => eprintln!("削除エラー: {}", e),
        }
    }
}

impl BookmarkHandler {
    pub fn new() -> Self {
        Self {
            bookmark_business: BookmarkBusinessLayer::new(),
        }
    }

    pub fn handle_bookmark_command(&self, action: Option<Actions>) -> Result<(), AppError> {
        match action {
            Some(Actions::Add { url }) => {
                if let Some(url) = url {
                    let result = self.bookmark_business.add_bookmark(&url)?;
                    println!("{} を追加しました", result);
                }
            }
            Some(Actions::Delete) => {
                let bookmarks = self.bookmark_business.list_bookmarks()?;
                let mut bookmarks_mut = bookmarks.clone();
                self.delete_prompt(&mut bookmarks_mut);
            }
            None => {
                let bookmarks = self.bookmark_business.list_bookmarks()?;
                for bookmark in bookmarks {
                    println!("{}", bookmark);
                }
            }
        }
        Ok(())
    }
}
