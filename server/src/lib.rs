use std::sync::Arc;

use actix_web::HttpServer;
use actix_web::{web, App as HttpApp};

use app::App;

use crate::router::{proxies, sources};

mod router;

pub async fn run() -> std::io::Result<()> {
    println!("restarted");
    let app = web::Data::new(App::new("mongodb://localhost/proxy-checker").await.unwrap());
    HttpServer::new(move || HttpApp::new().app_data(app.clone()).service(sources()).service(proxies()))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
