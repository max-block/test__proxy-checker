mod helper;
pub mod model;

pub use helper::bson_object_id_from_str;

use mongodb::options::ClientOptions;
use mongodb::Client;

use crate::db::helper::MongoCollection;
use crate::db::model::{Proxy, Source};
use crate::error::{AppError, AppResult};

pub struct Db {
    pub source: MongoCollection<Source>,
    pub proxy: MongoCollection<Proxy>,
}

impl Db {
    pub async fn new(database_url: &str) -> AppResult<Self> {
        let client = Client::with_uri_str(database_url).await?;
        let database_name = ClientOptions::parse(database_url)
            .await?
            .default_database
            .ok_or(AppError::EmptyDatabase)?;
        let database = client.database(&database_name);

        Ok(Self {
            source: MongoCollection::new(database.collection::<Source>("source")),
            proxy: MongoCollection::new(database.collection::<Proxy>("proxy")),
        })
    }
}
