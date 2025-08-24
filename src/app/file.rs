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
