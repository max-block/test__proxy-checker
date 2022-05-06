use actix_web::web::Data;
use actix_web::{get, post, web, Responder, Scope};
use bson::doc;

use app::App;

#[get("")]
async fn get_proxies(app: Data<App>) -> impl Responder {
    let r = app.db.proxy.find(doc! {}, "", 0).await.unwrap();
    serde_json::to_string(&r)
}

#[derive(serde::Deserialize, Debug)]
struct CreateProxyParams {
    name: String,
    value: String,
}

#[post("")]
async fn create_proxy(app: Data<App>, params: web::Json<CreateProxyParams>) -> impl Responder {
    dbg!(params);
    "done"
}

pub fn proxies() -> Scope {
    web::scope("/proxies").service(get_proxies).service(create_proxy)
}
