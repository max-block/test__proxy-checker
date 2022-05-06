use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

use chrono::Utc;
use futures::StreamExt;
use mongodb::bson::doc;

use app::App;
use app::db::bson_object_id_from_str;
use app::db::model::{Proxy, ProxyStatus, ProxyType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("mongodb://localhost/proxy-checker").await?;
    let p1 = Proxy::new(ProxyType::Socks5, "u2".into(), "p2".into(), "h2".into(), 999);
    let p2 = Proxy::new(ProxyType::Socks5, "u2".into(), "p2".into(), "h2".into(), 999);

    let r = app.db.proxy.insert_many(&vec![p1, p2]).await?;
    dbg!(r);

    let r = app.db.proxy.find_by_id(bson_object_id_from_str("6274e0c2852844f27114942e")?).await?;
    dbg!(r);

    let r = app.db.proxy.find(doc! {}, "", 0).await?;
    dbg!(r);
    Ok(())
}
