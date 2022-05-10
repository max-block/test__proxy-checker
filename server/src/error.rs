use std::fmt::{Display, Formatter};
use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("app error")]
    AppError(#[from] app::error::AppError),

    #[error("mongo error")]
    MongoError(#[from] mongodb::error::Error),
}


impl ResponseError for ServerError {
    
}