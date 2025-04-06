use std::io;

mod application;
mod domain;
mod dto;
mod infrastructure;
mod routes;
mod server;

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    server::run().await
}
