use actix_web::{delete, get, post, web, Responder, Scope};

#[get("")]
async fn get_sources() -> impl Responder {
    "todo"
}

#[post("")]
async fn create_source() -> impl Responder {
    "todo"
}

#[delete("")]
async fn delete_sources() -> impl Responder {
    "todo"
}

pub fn sources_scope() -> Scope {
    web::scope("/sources")
        .service(get_sources)
        .service(create_source)
        .service(delete_sources)
}
