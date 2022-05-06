use mongodb::bson;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("empty database")]
    EmptyDatabase,

    #[error("mongo error")]
    MongoError(#[from] mongodb::error::Error),

    #[error("parse objectId error")]
    ParseObjectId(#[from] bson::oid::Error),
}

pub type AppResult<T> = Result<T, AppError>;
