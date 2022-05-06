use actix_web::{get, post, web, Responder, Scope};

#[get("")]
async fn get_sources() -> impl Responder {
    "get_sources"
}

#[get("/test")]
async fn test() -> impl Responder {
    "test"
}

pub fn sources() -> Scope {
    web::scope("/sources").service(get_sources).service(test)
}
