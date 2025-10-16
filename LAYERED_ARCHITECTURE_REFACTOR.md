# Layered Architecture リファクタリング作業リスト

## 目標構造

```
src/
├── layers/
│   ├── presentation/
│   │   ├── cli/
│   │   │   ├── handlers/
│   │   │   │   ├── rss_handler.rs
│   │   │   │   ├── bookmark_handler.rs
│   │   │   │   ├── config_handler.rs
│   │   │   │   └── mod.rs
│   │   │   ├── display/
│   │   │   │   ├── terminal_display.rs
│   │   │   │   └── mod.rs
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── business/
│   │   ├── rss/
│   │   │   ├── rss_business.rs
│   │   │   ├── rss_parser.rs
│   │   │   └── mod.rs
│   │   ├── bookmark/
│   │   │   ├── bookmark_business.rs
│   │   │   └── mod.rs
│   │   ├── config/
│   │   │   ├── config_business.rs
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── data/
│   │   ├── repositories/
│   │   │   ├── config_repository.rs
│   │   │   ├── history_repository.rs
│   │   │   ├── file_repository.rs
│   │   │   └── mod.rs
│   │   └── mod.rs
│   └── mod.rs
├── models/
│   ├── article.rs     // 現在のEntity
│   ├── config.rs
│   ├── history.rs
│   ├── errors.rs
│   └── mod.rs
└── main.rs
```

## 段階的実装手順

### Phase 1: ディレクトリ構造作成
- [ ] `src/layers/` ディレクトリ構造作成
- [ ] `src/models/` ディレクトリ作成
- [ ] 各層の `mod.rs` ファイル作成

### Phase 2: Models層の実装
- [ ] `src/models/article.rs` - 現在の `Entity` を移動・リネーム
- [ ] `src/models/config.rs` - Config構造体の定義
- [ ] `src/models/history.rs` - History構造体の定義
- [ ] `src/models/errors.rs` - 共通エラー型定義
- [ ] `src/models/mod.rs` - モジュール公開

### Phase 3: Data Layer実装
- [ ] `src/layers/data/repositories/config_repository.rs`
  ```rust
  use crate::models::{config::Config, errors::DataError};
  
  pub struct ConfigRepository;
  
  impl ConfigRepository {
      pub fn load_config(&self) -> Result<Config, DataError> {
          // 現在のConfig::load()を移動
      }
      
      pub fn save_config(&self, config: &Config) -> Result<(), DataError> {
          // 現在のConfig::save()を移動
      }
  }
  ```

- [ ] `src/layers/data/repositories/history_repository.rs`
  ```rust
  use crate::models::{history::History, errors::DataError};
  
  pub struct HistoryRepository;
  
  impl HistoryRepository {
      pub fn load_history(&self) -> Result<History, DataError> {
          // 現在のHistory::load()を移動
      }
      
      pub fn save_history(&self, history: &History) -> Result<(), DataError> {
          // 現在のHistory::save()を移動
      }
  }
  ```

- [ ] `src/layers/data/repositories/file_repository.rs`
  ```rust
  pub struct FileRepository;
  
  impl FileRepository {
      pub fn read_file(&self, path: &str) -> Result<String, std::io::Error> {
          // ファイル読み込み処理
      }
      
      pub fn write_file(&self, path: &str, content: &str) -> Result<(), std::io::Error> {
          // ファイル書き込み処理
      }
  }
  ```

### Phase 4: Business Layer実装
- [ ] `src/layers/business/rss/rss_business.rs`
  ```rust
  use crate::layers::data::repositories::{ConfigRepository, HistoryRepository};
  use crate::models::{article::Article, errors::BusinessError};
  
  pub struct RssBusinessLayer {
      config_repo: ConfigRepository,
      history_repo: HistoryRepository,
  }
  
  impl RssBusinessLayer {
      pub fn new() -> Self {
          Self {
              config_repo: ConfigRepository,
              history_repo: HistoryRepository,
          }
      }
      
      pub async fn fetch_articles(&self, options: HashMap<String, String>) -> Result<Vec<Article>, BusinessError> {
          // 現在のRssController::index()のビジネスロジック部分を移動
          // 1. 設定読み込み
          // 2. RSS取得
          // 3. XML解析
          // 4. 新しい記事フィルタリング
          // 5. 履歴更新
      }
      
      pub fn add_feed(&self, url: &str) -> Result<String, BusinessError> {
          // 現在のRssController::add_link()を移動
      }
      
      pub fn remove_feed(&self) -> Result<(), BusinessError> {
          // 現在のRssController::delete_link()を移動
      }
      
      pub fn list_feeds(&self) -> Result<Vec<String>, BusinessError> {
          // 現在のRssController::show()を移動
      }
  }
  ```

- [ ] `src/layers/business/bookmark/bookmark_business.rs`
  ```rust
  pub struct BookmarkBusinessLayer {
      config_repo: ConfigRepository,
  }
  
  // BookmarkControllerのロジックを移動
  ```

- [ ] `src/layers/business/config/config_business.rs`
  ```rust
  pub struct ConfigBusinessLayer {
      config_repo: ConfigRepository,
  }
  
  // ConfigControllerのロジックを移動
  ```

### Phase 5: Presentation Layer実装
- [ ] `src/layers/presentation/cli/handlers/rss_handler.rs`
  ```rust
  use crate::layers::business::rss::RssBusinessLayer;
  use crate::commands::Actions;
  
  pub struct RssHandler {
      rss_business: RssBusinessLayer,
  }
  
  impl RssHandler {
      pub fn new() -> Self {
          Self {
              rss_business: RssBusinessLayer::new(),
          }
      }
      
      pub async fn handle_rss_command(&self, action: Option<Actions>, options: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
          match action {
              Some(Actions::Add { url }) => {
                  if let Some(url) = url {
                      let result = self.rss_business.add_feed(url).await?;
                      println!("{} を追加しました", result);
                  }
              }
              Some(Actions::Delete) => {
                  self.rss_business.remove_feed().await?;
                  println!("URLを削除しました");
              }
              None => {
                  let articles = self.rss_business.fetch_articles(options).await?;
                  // 表示処理は別のDisplayハンドラーに委譲
              }
          }
          Ok(())
      }
  }
  ```

- [ ] `src/layers/presentation/cli/display/terminal_display.rs`
  ```rust
  use crate::models::article::Article;
  
  pub struct TerminalDisplay;
  
  impl TerminalDisplay {
      pub fn display_articles(&self, articles: &[Article], options: HashMap<String, String>) {
          // 現在のscreen::Screen::draw()を移動
      }
      
      pub fn display_message(&self, message: &str) {
          println!("{}", message);
      }
  }
  ```

- [ ] 他のハンドラー（bookmark_handler.rs、config_handler.rs）も同様に実装

### Phase 6: main.rsの更新
- [ ] `src/main.rs` を更新
  ```rust
  mod layers;
  mod models;
  mod commands;
  
  use std::collections::HashMap;
  use clap::Parser;
  use commands::{Actions, Commands};
  use layers::presentation::cli::handlers::{RssHandler, BookmarkHandler, ConfigHandler};
  
  #[derive(Parser, Debug)]
  #[command(version, about, long_about = None)]
  #[clap(name = "nnm", version = "1.0.1", about = "コンソールで読むRSSリーダー")]
  struct Cli {
      #[command(subcommand)]
      command: Option<Commands>,
      
      #[arg(short, long, default_value_t = 10)]
      number: i32,
  }
  
  #[tokio::main]
  async fn main() {
      let cli = Cli::parse();
      let number = cli.number;
      let mut options = HashMap::new();
      options.insert("head".to_string(), number.to_string());
      
      let rss_handler = RssHandler::new();
      let bookmark_handler = BookmarkHandler::new();
      let config_handler = ConfigHandler::new();
      
      let result = match &cli.command {
          Some(Commands::Init) => {
              config_handler.handle_init().await
          }
          Some(Commands::Rss { action }) => {
              rss_handler.handle_rss_command(action.clone(), options).await
          }
          Some(Commands::Bookmark { action }) => {
              bookmark_handler.handle_bookmark_command(action.clone()).await
          }
          Some(Commands::History) => {
              // history_handler.handle_history().await
              Ok(())
          }
          None => {
              rss_handler.handle_rss_command(None, options).await
          }
      };
      
      if let Err(e) = result {
          eprintln!("Error: {:#?}", e);
      }
  }
  ```

### Phase 7: 既存コードのクリーンアップ
- [ ] `src/app/` 配下の古いファイルを段階的に削除
- [ ] 古い `mod app;` を `mod layers; mod models;` に変更
- [ ] 不要になったコントローラファイルを削除

### Phase 8: テストとビルド
- [ ] `cargo build` でコンパイルエラーを修正
- [ ] `cargo run` で基本動作確認
- [ ] 各コマンドの動作テスト
  - [ ] `cargo run -- init`
  - [ ] `cargo run -- rss add "https://example.com/feed"`
  - [ ] `cargo run -- rss`
  - [ ] `cargo run -- bookmark add "https://example.com"`
  - [ ] `cargo run -- history`

## 注意事項

1. **段階的実装**: 一度に全て変更せず、layer単位で実装・テストする
2. **既存機能の保持**: リファクタリング中も既存機能が動作することを確認
3. **エラーハンドリング**: 各層で適切なエラー型を定義・使用
4. **依存関係**: 上位層→下位層の依存のみ許可（Presentation→Business→Data）
5. **テスト**: 各層が完成したら個別にテスト可能

## 完了後の効果

- **責務の分離**: 各層が明確な責務を持つ
- **テスタビリティ**: 層ごとに独立してテスト可能
- **保守性**: 変更箇所の影響範囲が限定される
- **再利用性**: Business層は他のPresentation層からも利用可能