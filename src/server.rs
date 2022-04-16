use actix_web::{App, HttpServer, web};

use crate::handler::{index_handler, link_handler};

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index_handler::index))
            .default_service(web::get().to(link_handler::link))
    });
    println!("Running server on http://localhost:8080");
    server.bind(("0.0.0.0", 8080))?.run().await
}
