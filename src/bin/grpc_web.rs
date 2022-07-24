use std::env;
use std::error::Error;

use sqlx::mysql::MySqlPoolOptions;
use tonic::transport::Server;

use kingsol::grpc_api::GrpcApi;
use kingsol::kingsol_api::kingsol_api_server::KingsolApiServer;

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

    let port: u16 = env::var("GRPC_WEB_PORT")?.parse()?;
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let api = GrpcApi::new(pool);
    let server = KingsolApiServer::new(api);
    let service = tonic_web::config().allow_all_origins().enable(server);

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
