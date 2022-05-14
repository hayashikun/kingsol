use actix_web::{App, HttpServer, web};
use anyhow::{Context, Result};

use kingsol::redis::create_connection_pool;
use kingsol::web_controller::link_handler;

#[actix_web::main]
pub async fn main() -> Result<()> {
    let redis_pool = create_connection_pool("redis://localhost:6379").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .default_service(web::get().to(link_handler))
    });
    println!("Running server on http://localhost:8080");
    server.bind(("0.0.0.0", 8080))?.run().await.context("Failed to start server")
}
