use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("正規表現エラー: {0}")]
    RegexError(#[from] regex::Error),

    #[error("XML解析エラー: {0}")]
    XmlError(#[from] quick_xml::DeError),

    #[error("ファイル入出力エラー: {0}")]
    FileError(#[from] std::io::Error),

    #[error("パースエラー: {0}")]
    ParseError(String),
}

// app::error::AppError からの変換
impl From<crate::app::error::AppError> for AppError {
    fn from(err: crate::app::error::AppError) -> Self {
        match err {
            crate::app::error::AppError::RegexError(e) => AppError::RegexError(e),
            crate::app::error::AppError::XmlError(e) => AppError::XmlError(e),
            crate::app::error::AppError::FileError(e) => AppError::FileError(e),
            crate::app::error::AppError::ParseError(s) => AppError::ParseError(s),
        }
    }
}
