use crate::app::App;
use crate::route::proxy::proxies_scope;
use crate::route::source::sources_scope;
use actix_web::middleware::Logger;
use actix_web::{web, App as HttpApp, HttpServer};

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app = web::Data::new(App::new("mongodb://localhost/proxy-checker").await.unwrap());
    HttpServer::new(move || {
        HttpApp::new()
            .wrap(Logger::default())
            .app_data(app.clone())
            .service(sources_scope())
            .service(proxies_scope())
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
