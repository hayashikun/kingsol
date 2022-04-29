use tonic::{Request, Response, Status, transport::Server};

use kingsol::grpc::API;
use kingsol::proto::kingsol::kingsol_api_server::KingsolApiServer;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8081".parse().unwrap();

    let api = KingsolApiServer::new(API::default());
    let service = tonic_web::config()
        .allow_origins(vec!["127.0.0.1"])
        .enable(api);

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
