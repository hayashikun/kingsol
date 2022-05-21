use tonic::transport::Server;

use kingsol::grpc_api::GrpcApi;
use kingsol::kingsol_api::kingsol_api_server::KingsolApiServer;
use kingsol::redis::create_connection_pool;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_pool = create_connection_pool("redis://localhost:6379").unwrap();

    let addr = "0.0.0.0:8081".parse().unwrap();
    let api = GrpcApi::new(redis_pool);
    let api = KingsolApiServer::new(api);
    let api = tonic_web::config()
        .allow_all_origins()
        .enable(api);

    println!("Server listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(api)
        .serve(addr)
        .await?;

    Ok(())
}
