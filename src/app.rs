use crate::db::Db;
use std::sync::Arc;

use crate::error::AppResult;
use crate::service::{ProxyService, SourceService};

pub struct App {
    pub db: Arc<Db>,
    pub proxy_service: ProxyService,
    pub source_service: SourceService,
}

impl App {
    pub async fn new(database_url: &str) -> AppResult<Self> {
        let db = Arc::new(Db::new(database_url).await?);
        Ok(Self {
            proxy_service: ProxyService::new(db.clone()),
            source_service: SourceService::new(db.clone()),
            db,
        })
    }
}
