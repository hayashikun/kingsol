use std::env;
use tonic::transport::Server;

use anyhow::{Context, Result};

use kingsol::grpc_api::GrpcApi;
use kingsol::grpc_interceptor::GrpcInterceptor;
use kingsol::kingsol_api::kingsol_api_server::KingsolApiServer;
use kingsol::redis::create_connection_pool;

#[tokio::main]
pub async fn main() -> Result<()> {
    let redis_url = env::var("REDIS_URL")?;
    let redis_pool =
        create_connection_pool(redis_url).context("Failed to create redis connection pool")?;

    let port: u16 = env::var("PORT")?.parse()?;
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let api = GrpcApi::new(redis_pool.clone());
    let interceptor = GrpcInterceptor::new(redis_pool);
    let server = KingsolApiServer::with_interceptor(api, interceptor);
    let service = tonic_web::config().allow_all_origins().enable(server);

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
