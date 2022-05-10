use crate::app::App;
use crate::service::CreateProxyParams;
use actix_web::web::{Data, Json};
use actix_web::{delete, get, post, web, Scope};
use mongodb::bson::{doc, Bson};
use serde_json::{json, Value};

use crate::error::AppError;

type JsonResult = Result<Json<Value>, AppError>;
type BsonResult = Result<Json<Bson>, AppError>;

#[get("")]
async fn get_proxies(app: Data<App>) -> JsonResult {
    println!("z1");
    let res = app.db.proxy.find(doc! {}, "", 1).await?;
    println!("z2");
    Ok(Json(serde_json::to_value(res).unwrap()))
}

#[post("")]
async fn create_proxy(app: Data<App>, params: Json<CreateProxyParams>) -> BsonResult {
    let res = app.proxy_service.create(params.0).await?;
    Ok(Json(res))
}

#[delete("")]
async fn delete_proxies(app: Data<App>) -> JsonResult {
    let res = app.db.proxy.delete_many(doc! {}).await?;
    Ok(Json(json!({ "deleted": res })))
}

pub fn proxies_scope() -> Scope {
    web::scope("/proxies")
        .service(get_proxies)
        .service(create_proxy)
        .service(delete_proxies)
}
