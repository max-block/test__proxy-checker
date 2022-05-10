use std::sync::Arc;
use serde::{Deserialize, Serialize};

use chrono::Utc;
use mongodb::bson::Bson;

use crate::{
    db::{
        bson_object_id_from_str,
        model::{Proxy, ProxyStatus, ProxyType},
        Db,
    },
    error::AppError,
};

pub struct ProxyService {
    db: Arc<Db>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProxyParams {
    pub r#type: ProxyType,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u32,
}

impl From<CreateProxyParams> for Proxy {
    fn from(params: CreateProxyParams) -> Self {
        Self {
            id: None,
            r#type: params.r#type,
            status: ProxyStatus::Unknown,
            username: params.username,
            password: params.password,
            host: params.host,
            port: params.port,
            created_at: Utc::now(),
            checked_at: None,
            last_ok_at: None,
        }
    }
}

impl ProxyService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn check(&self, id: String) -> Result<ProxyStatus, AppError> {
        let res = self.db.proxy.find_by_id(bson_object_id_from_str(id)?).await?;
        if res.is_none() {
            return Err(AppError::ProxyNotFound);
        }

        todo!()
    }

    pub async fn create(&self, params: CreateProxyParams) ->Result<Bson, AppError>{
        Ok(self.db.proxy.insert(&params.into()).await?)
    }
}
