use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::{Context, Result};
use r2d2::Pool;
use redis::Client;
use std::env;

use kingsol::interactor::GetLink;
use kingsol::redis::create_connection_pool;
use kingsol::redis_repository::RedisRepository;
use kingsol::web_controller::LinkController;

#[actix_web::main]
pub async fn main() -> Result<()> {
    let redis_url = env::var("REDIS_URL")?;
    let redis_pool =
        create_connection_pool(redis_url).context("Failed to create redis connection pool")?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .default_service(web::get().to(link_handler))
    });

    let port: u16 = env::var("PORT")?.parse()?;
    println!("Running server on http://localhost:{}", port);
    server
        .bind(("0.0.0.0", port))?
        .run()
        .await
        .context("Failed to start server")
}

async fn link_handler(req: HttpRequest, pool: web::Data<Pool<Client>>) -> HttpResponse {
    let repository = RedisRepository::new(pool.get_ref());
    if let Err(_) = repository {
        return HttpResponse::InternalServerError().finish();
    }
    let mut repository = repository.unwrap();
    let get_link = GetLink::new(&mut repository);

    let mut controller = LinkController::new(get_link);
    controller.handle(req)
}
