use actix_web::web::Data;
use actix_web::{get, post, web, Responder, Scope, delete};
use app::service::proxy::CreateProxyParams;
use bson::doc;
use serde_json::json;

use app::App;

use crate::error::ServerError;

#[get("")]
async fn get_proxies(app: Data<App>) -> impl Responder {
    let r = app.db.proxy.find(doc! {}, "", 0).await.unwrap();
    serde_json::to_string(&r)
}

#[post("")]
async fn create_proxy(app: Data<App>, params: web::Json<CreateProxyParams>) -> Result<impl Responder, ServerError> {
    let res = app.proxy_service.create(params.0).await?;
    dbg!(&res);
    Ok("done")
}

#[delete("")]
async fn delete_all_proxies(app: Data<App>) -> Result<impl Responder, ServerError> {
    let res =app.db.proxy.delete_many(doc! {}).await?;
    Ok(json!{"deleted": res})
}

pub fn proxies() -> Scope {
    web::scope("/proxies").service(get_proxies).service(create_proxy)
}
