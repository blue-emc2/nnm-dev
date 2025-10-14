use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

pub struct FileRepository;

impl FileRepository {
    pub fn new() -> Self {
        FileRepository
    }

    /// ファイルを読み込む
    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }

    /// ファイルに書き込む
    pub fn write_file<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<(), std::io::Error> {
        // ディレクトリが存在しない場合は作成
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(path)?;
        write!(file, "{}", content)?;
        Ok(())
    }

    /// JSON形式でシリアライズしてファイルに保存
    pub fn save_json<T: Serialize, P: AsRef<Path>>(&self, path: P, content: &T) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(content)?;
        self.write_file(path, &json)
    }

    /// JSONファイルを読み込んでデシリアライズ
    pub fn load_json<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(&self, path: P) -> Result<T, std::io::Error> {
        let content = self.read_file(path)?;
        let data = serde_json::from_str(&content)?;
        Ok(data)
    }

    /// ディレクトリを作成
    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        fs::create_dir_all(path)
    }

    /// ファイルが存在するかチェック
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }
}
