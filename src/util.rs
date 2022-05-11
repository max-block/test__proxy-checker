use actix_web::web::Json;
use serde::Serialize;
use serde_json::Value;

use crate::error::AppResult;

pub type JsonResult = AppResult<Json<Value>>;

pub fn json_result<T: Serialize>(value: T) -> JsonResult {
    Ok(Json(serde_json::to_value(value).unwrap())) // TODO: replace unwrap with AppError
}
