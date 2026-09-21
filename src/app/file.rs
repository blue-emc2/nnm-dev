use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

pub trait File {
    fn file_path(&self) -> PathBuf;

    fn save_to_file<T: Serialize>(&self, content: T) -> Result<(), std::io::Error> {
        let path = self.file_path();

        // ディレクトリが存在しない場合は作成
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_json = serde_json::to_string_pretty(&content)?;
        let mut file = std::fs::File::create(path)?;
        write!(file, "{}", config_json)?;
        Ok(())
    }

    fn load_from_file<T: for<'de> Deserialize<'de>>(&self) -> io::Result<T> {
        let path = self.file_path();
        let config = serde_json::from_str(&fs::read_to_string(path)?)?;
        Ok(config)
    }
}

pub fn app_config_dir() -> PathBuf {
    // XDG_CONFIG_HOMEが設定されていれば、そこが設定パス
    // 未設定の場合は、~/.configが返る
    let root_config_path = match std::env::var("XDG_CONFIG_HOME") {
        Ok(val) => Some(PathBuf::from(val)),
        Err(_) => std::env::home_dir().map(|home| home.join(".config")),
    };
    root_config_path
        .unwrap_or_else(|| PathBuf::from("."))
        .join("nnm")
}
