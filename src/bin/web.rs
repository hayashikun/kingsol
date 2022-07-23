use std::env;
use std::error::Error;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

use kingsol::interactor::GetLink;
use kingsol::mysql_repository::MysqlRepository;
use kingsol::web_controller::LinkController;

#[tokio::main]
pub async fn main() {
    if let Err(e) = _main().await {
        println!("{}", e);
    }
}

async fn _main() -> Result<(), Box<dyn Error>> {
    let mysql_url = env::var("MYSQL_URL")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(mysql_url.as_str())
        .await?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .default_service(web::get().to(link_handler))
    });

    let port: u16 = env::var("WEB_PORT")?.parse()?;
    println!("Running server on 0.0.0.0:{}", port);
    server.bind(("0.0.0.0", port))?.run().await?;

    Ok(())
}

async fn link_handler(req: HttpRequest, pool: web::Data<Pool<MySql>>) -> HttpResponse {
    let conn = pool.get_ref();
    let repository = MysqlRepository::new(conn);
    let get_link = GetLink::new(repository);

    let mut controller = LinkController::new(get_link);
    controller.handle(req).await
}
