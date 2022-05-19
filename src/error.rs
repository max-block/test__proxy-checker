use actix_web::ResponseError;
use mongodb::bson;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("empty database")]
    EmptyDatabase,

    #[error("mongo error")]
    MongoError(#[from] mongodb::error::Error),

    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("parse objectId error")]
    ParseObjectId(#[from] bson::oid::Error),

    #[error("not found")]
    NotFound,

    

}

impl ResponseError for AppError {}

pub type AppResult<T> = Result<T, AppError>;