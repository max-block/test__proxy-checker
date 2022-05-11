use chrono::Utc;
use mongodb::bson::Bson;
use serde::{Serialize, Deserialize};

use crate::{
    db::{bson_object_id_from_str, model::Source, Db},
    error::AppResult,
};
use std::sync::Arc;

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
        let _source = self.db.source.find_by_id(id.into()).await?;
        dbg!(_source);
        Ok(())
    }

    pub async fn create(&self, params: CreateSourceParams) -> AppResult<Bson> {
        Ok(self.db.source.insert(&params.into()).await?)
    }
}
