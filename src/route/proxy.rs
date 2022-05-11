use crate::app::App;
use crate::service::CreateProxyParams;
use crate::util::{json_result, JsonResult};
use actix_web::web::{Data, Json};
use actix_web::{delete, get, post, web, Scope};
use mongodb::bson::doc;

#[get("")]
async fn get_proxies(app: Data<App>) -> JsonResult {
    json_result(app.db.proxy.find(doc! {}, "", 1).await?)
}

#[post("")]
async fn create_proxy(app: Data<App>, params: Json<CreateProxyParams>) -> JsonResult {
    json_result(app.proxy_service.create(params.0).await?)
}

#[delete("")]
async fn delete_proxies(app: Data<App>) -> JsonResult {
    json_result(app.db.proxy.delete_many(doc! {}).await?)
}

pub fn proxies_scope() -> Scope {
    web::scope("/proxies")
        .service(get_proxies)
        .service(create_proxy)
        .service(delete_proxies)
}
