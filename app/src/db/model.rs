use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyType {
    #[serde(rename_all = "lowercase")]
    Socks5,
    #[serde(rename_all = "lowercase")]
    Html,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyStatus {
    #[serde(rename_all = "lowercase")]
    Unknown,
    #[serde(rename_all = "lowercase")]
    Ok,
    #[serde(rename_all = "lowercase")]
    Down,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proxy {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub r#type: ProxyType,
    pub status: ProxyStatus,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub created_at: DateTime<Utc>,
    pub checked_at: Option<DateTime<Utc>>,
    pub last_ok_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub value: String,
    pub tags: Vec<String>,
}

impl Proxy {
    pub fn new(r#type: ProxyType, username: String, password: String, host: String, port: u32) -> Self {
        Self {
            id: None,
            r#type,
            status: ProxyStatus::Unknown,
            username,
            password,
            host,
            port,
            created_at: Utc::now(),
            checked_at: None,
            last_ok_at: None,
        }
    }
}
