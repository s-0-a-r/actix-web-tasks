use actix_web::{App, HttpServer};
use std::io;

use crate::routes;

pub async fn run() -> io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
