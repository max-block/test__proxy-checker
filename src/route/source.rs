use actix_web::{
    delete, get, post,
    web::{self, Data, Json, Path},
    Responder, Scope,
};
use mongodb::bson::doc;

use crate::{
    app::App,
    service::CreateSourceParams,
    util::{json_result, JsonResult}
};

#[get("")]
async fn get_sources(app: Data<App>) -> JsonResult {
    json_result(app.db.source.find(doc! {}, "_id", 0).await?)
}


#[post("")]
async fn create_source(app: Data<App>, params: Json<CreateSourceParams>) -> JsonResult {
    json_result(app.source_service.create(params.0).await?)
}

#[delete("")]
async fn delete_all_sources() -> impl Responder {
    "todo"
}

#[post("/{pk}/check")]
async fn check_source(app: Data<App>, path: Path<(String,)>) ->JsonResult {
    json_result(app.source_service.check(path.into_inner().0).await?)
}


#[get("/{pk}")]
async fn get_source(app: Data<App>, path: Path<(String,)>) ->JsonResult {
    json_result(app.db.source.find_by_id(path.into_inner().0.into()).await?)
}


#[delete("/{pk}")]
async fn delete_source(app: Data<App>, path: Path<(String,)>) -> JsonResult {
    json_result(app.db.source.delete_by_id(path.into_inner().0.into()).await?)
}

pub fn sources_scope() -> Scope {
    web::scope("/sources")
        .service(get_sources)
        .service(get_source)
        .service(create_source)
        .service(check_source)
        .service(delete_all_sources)
        .service(delete_source)
}
