use crate::layers::business::config::ConfigBusinessLayer;
use crate::models::config::ConfigMessage;
use crate::models::errors::AppError;

pub struct ConfigHandler {
    config_business: ConfigBusinessLayer,
}

impl ConfigHandler {
    pub fn new() -> Self {
        Self {
            config_business: ConfigBusinessLayer::new(),
        }
    }

    pub fn handle_init(&self) -> Result<(), AppError> {
        match self.config_business.create()? {
            ConfigMessage::Success(path) => {
                println!("設定ファイルを作成しました。{}", path);
                println!("nnm rss add \"{{url}}\" でRSSのURLを追加しましょう。");
            }
            ConfigMessage::ExistsConfig => {
                println!("設定ファイルはすでに存在します。");
            }
        }
        Ok(())
    }
}
