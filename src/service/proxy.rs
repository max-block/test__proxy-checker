use serde::{Deserialize, Serialize};
use std::sync::Arc;

use chrono::Utc;
use mongodb::bson::Bson;
use crate::db::{bson_object_id_from_str, Db};
use crate::db::model::{Proxy, ProxyStatus};
use crate::error::AppError;

pub struct ProxyService {
    db: Arc<Db>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProxyParams {
    pub url: String,
    pub tags: Vec<String>,
}

impl From<CreateProxyParams> for Proxy {
    fn from(params: CreateProxyParams) -> Self {
        Self {
            id: None,
            url: params.url,
            tags: params.tags,
            status: ProxyStatus::Unknown,
            created_at: Utc::now(),
            checked_at: None,
            last_ok_at: None
        }
    }
}

impl ProxyService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn check(&self, id: String) -> Result<ProxyStatus,   AppError> {
        let res = self.db.proxy.find_by_id(bson_object_id_from_str(id)?).await?;
        if res.is_none() {
            return Err(AppError::NotFound);
        }

        todo!()
    }

    pub async fn create(&self, params: CreateProxyParams) -> Result<Bson, AppError> {
        Ok(self.db.proxy.insert(&params.into()).await?)
    }
}

