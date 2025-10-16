// RSS business logic
use crate::layers::data::repositories::config_repository::ConfigRepository;
use crate::layers::data::repositories::history_repository::HistoryRepository;
use crate::models::article::Article;
use crate::models::errors::AppError;
use crate::app::parser::Parser;

pub struct RssBusinessLayer {
    config_repo: ConfigRepository,
    history_repo: HistoryRepository,
}

impl RssBusinessLayer {
    pub fn new() -> Self {
        Self {
            config_repo: ConfigRepository::new(),
            history_repo: HistoryRepository::new(),
        }
    }

    /// RSSフィードを追加
    pub fn add_feed(&self, url: &str) -> Result<String, AppError> {
        let mut config = self.config_repo.load()?;
        let links = config.links();

        if links.contains(&url.to_string()) {
            return Ok(url.to_string());
        }

        let links = config.mut_links();
        links.push(url.to_string());
        self.config_repo.save(&config)?;
        Ok(url.to_string())
    }

    /// RSSフィードを削除
    pub fn delete_feed(&self, url: &str) -> Result<(), AppError> {
        let mut config = self.config_repo.load()?;
        let links = config.mut_links();

        let index = links
            .iter()
            .position(|x| x == url)
            .ok_or_else(|| AppError::ParseError("URLが見つかりません".to_string()))?;

        links.remove(index);
        self.config_repo.save(&config)?;
        Ok(())
    }

    /// RSSフィード一覧を取得
    pub fn list_feeds(&self) -> Result<Vec<String>, AppError> {
        let config = self.config_repo.load()?;
        Ok(config.links().clone())
    }

    /// RSSフィードを取得して記事を返す
    pub async fn fetch_articles(&self) -> Result<Vec<Article>, AppError> {
        let config = self.config_repo.load()?;
        let links = config.links().clone();
        let chunk_size = config.chunk_size();

        // 並行でRSSを取得
        let tasks = links.into_iter().map(|link| {
            tokio::spawn(async move {
                #[cfg(debug_assertions)]
                {
                    println!(
                        "- start fetch task {} : {:?}",
                        link,
                        std::thread::current().id()
                    );
                }
                let res = Self::fetch_rss(link.clone()).await;
                #[cfg(debug_assertions)]
                {
                    println!(
                        "- end fetch task {} : {:?}",
                        link,
                        std::thread::current().id()
                    );
                }
                res
            })
        });

        let results = futures::future::join_all(tasks).await;
        let fetched_data: Vec<String> = results
            .into_iter()
            .filter_map(|res| res.ok())
            .filter_map(|res| res.ok())
            .collect();

        // XMLをパース
        let mut entities = self.parse_xml(fetched_data, chunk_size)?;

        // 新しい記事のみをフィルタリング
        self.filter_new_entities(&mut entities)?;

        // 履歴を保存
        if !entities.is_empty() {
            self.save_history(&entities)?;
        }

        Ok(entities)
    }

    async fn fetch_rss(url: String) -> Result<String, reqwest::Error> {
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        Ok(body)
    }

    fn parse_xml(&self, bodies: Vec<String>, chunk_size: i32) -> Result<Vec<Article>, AppError> {
        let parser = Parser::new()?;
        let mut entities = Vec::new();

        for body in bodies {
            let parsed_entities = parser.parse(body)?;
            // Entity を Article に変換
            let mut chunks: Vec<Article> = parsed_entities
                .into_iter()
                .take(chunk_size.try_into().unwrap())
                .map(|entity| {
                    use crate::models::article::EntityType as ArticleEntityType;
                    use crate::app::entity::EntityType as AppEntityType;

                    let article_type = match entity.entity_type {
                        AppEntityType::Rdf => ArticleEntityType::Rdf,
                        AppEntityType::Rss => ArticleEntityType::Rss,
                        AppEntityType::Atom => ArticleEntityType::Atom,
                        AppEntityType::Unknown => ArticleEntityType::Unknown,
                    };

                    Article {
                        entity_type: article_type,
                        title: entity.title,
                        link: entity.link,
                        description: entity.description,
                        pub_date: entity.pub_date,
                    }
                })
                .collect();
            entities.append(&mut chunks);
        }

        Ok(entities)
    }

    fn filter_new_entities(&self, entities: &mut Vec<Article>) -> Result<(), AppError> {
        let history = self.history_repo.load()?;
        let history_entities = history.get_entities();

        entities.retain(|entity| {
            !history_entities.iter().any(|h| h.link == entity.link)
        });

        Ok(())
    }

    fn save_history(&self, entities: &[Article]) -> Result<(), AppError> {
        let mut history = self.history_repo.load().unwrap_or_else(|_| {
            eprintln!("履歴ファイルが見つかりませんでした。\nhistory.jsonを再作成します。");
            crate::models::history::History::new()
        });

        for entity in entities {
            let article = Article {
                entity_type: entity.entity_type.clone(),
                title: entity.title.clone(),
                link: entity.link.clone(),
                description: entity.description.clone(),
                pub_date: None,
            };
            history.entity_push(article);
        }

        history.update_last_fetched_date();
        self.history_repo.save(&history)?;

        Ok(())
    }
}
