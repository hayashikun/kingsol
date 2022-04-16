use actix_web::{App, HttpServer, web};
use anyhow::{Context, Result};

use crate::handler::{action_handler, link_handler};
use crate::redis::create_connection_pool;

#[actix_web::main]
pub async fn start() -> Result<()> {
    let redis_pool = create_connection_pool("redis://localhost:6379").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .service(action_handler::add)
            .service(action_handler::list)
            .default_service(web::get().to(link_handler::link))
    });
    println!("Running server on http://localhost:8080");
    server.bind(("0.0.0.0", 8080))?.run().await.context("Failed to start server")
}
