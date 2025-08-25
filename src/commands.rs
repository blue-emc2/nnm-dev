use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 設定ファイルを作成します。
    Init,
    /// RSSの追加、削除を行います。
    Rss {
        #[command(subcommand)]
        action: Option<Actions>,
    },
    /// お気に入りの追加、削除を行います。
    Bookmark {
        #[command(subcommand)]
        action: Option<Actions>,
    },
    /// 履歴の表示を行います。
    History,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Actions {
    Add { url: Option<String> },
    Delete,
}
