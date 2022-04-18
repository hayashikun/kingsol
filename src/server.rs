use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::http::header;
use anyhow::{Context, Result};

use crate::handler::{action_handler, link_handler};
use crate::redis::create_connection_pool;

#[actix_web::main]
pub async fn start() -> Result<()> {
    let redis_pool = create_connection_pool("redis://localhost:6379").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .route("/", web::to(|| async {
                HttpResponse::TemporaryRedirect().append_header((header::LOCATION, "/!/list")).finish()
            }))
            .service(action_handler::create)
            .service(action_handler::list)
            .default_service(web::get().to(link_handler::link))
    });
    println!("Running server on http://localhost:8080");
    server.bind(("0.0.0.0", 8080))?.run().await.context("Failed to start server")
}
