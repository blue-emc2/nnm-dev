use thiserror::Error;

#[derive(Error,Debug)]
pub enum AppError {
  #[error("正規表現エラー: {0}")]
  RegexError(#[from] regex::Error),
}
