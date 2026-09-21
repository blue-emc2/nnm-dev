mod app;
mod commands;
mod layers;
mod models;

use std::collections::HashMap;

use clap::Parser;
use commands::{Actions, Commands};
use layers::presentation::cli::handlers::{
    bookmark_handler::BookmarkHandler, config_handler::ConfigHandler, rss_handler::RssHandler,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(name = "nnm", version = "1.0.1", about = "コンソールで読むRSSリーダー")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// A numeric option
    #[arg(short, long, default_value_t = 10)]
    number: i32,
}

fn main() {
    let cli = Cli::parse();
    let number = cli.number;
    let mut options = HashMap::new();
    options.insert("head".to_string(), number.to_string());

    let config_handler = ConfigHandler::new();
    let bookmark_handler = BookmarkHandler::new();
    let rss_handler = RssHandler::new();

    let result = match &cli.command {
        Some(Commands::Init) => config_handler.handle_init(),
        Some(Commands::Rss { action }) => rss_handler.handle_rss_command(action.clone(), options),
        Some(Commands::Bookmark { action }) => {
            bookmark_handler.handle_bookmark_command(action.clone())
        }
        Some(Commands::History) => {
            // TODO: History handler implementation
            println!("History機能は未実装です");
            Ok(())
        }
        None => rss_handler.handle_rss_command(None, options),
    };

    if let Err(e) = result {
        eprintln!("Error: {:#?}", e);
    }
}
