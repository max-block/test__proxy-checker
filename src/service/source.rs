use chrono::Utc;
use mongodb::bson::Bson;
use serde::{Serialize, Deserialize};

use crate::{
    db::{ model::Source, Db},
    error::{AppResult, AppError},
};
use std::{sync::Arc, net::Ipv4Addr, str::FromStr};

pub struct SourceService {
    db: Arc<Db>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSourceParams {
    pub id: String,
    pub url: String,
    pub protocol: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub port: Option<u16>,
}

impl From<CreateSourceParams> for Source {
    fn from(params: CreateSourceParams) -> Self {
        Source {
            id: params.id,
            url: params.url,
            protocol: params.protocol,
            username: params.username,
            password: params.password,
            port: params.port,
            created_at: Utc::now(),
            checked_at: None,
        }
    }
}

impl SourceService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn check(&self, id: String) -> AppResult<()> {
        let source = self.db.source.find_by_id(id.into()).await?.ok_or(AppError::NotFound)?;
        let res = reqwest::get(source.url).await?.text().await?;
        let ip_list = Self::search_ip(res);

        dbg!(ip_list);
        Ok(())
    }

    pub async fn create(&self, params: CreateSourceParams) -> AppResult<Bson> {
        Ok(self.db.source.insert(&params.into()).await?)
    }

    fn search_ip(data: String) -> Vec<String> {
        let mut result = vec![];    
        for word in data.split_whitespace() {
            if let Ok(_) = Ipv4Addr::from_str(&word) {
                result.push(word.into());
            }
        }
        result
    }
}



