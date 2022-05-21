use r2d2::Pool;
use redis::Client;
use tonic::{Request, Response, Status};

use crate::grpc_controller::{GrpcCreateController, GrpcGetController, GrpcListController};
use crate::interactor::{CreateLink, GetLink, ListLinks};
use crate::kingsol_api::kingsol_api_server::KingsolApi;
use crate::kingsol_api::{
    CreateRequest, CreateResponse, GetRequest, GetResponse, ListRequest, ListResponse,
};
use crate::redis_repository::RedisRepository;

pub struct GrpcApi {
    redis_pool: Pool<Client>,
}

impl GrpcApi {
    pub fn new(redis_pool: Pool<Client>) -> Self {
        Self { redis_pool }
    }
}

#[tonic::async_trait]
impl KingsolApi for GrpcApi {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let mut repository = RedisRepository::new(&self.redis_pool)?;
        let get_link = GetLink::new(&mut repository);
        let mut controller = GrpcGetController::new(get_link);
        controller.handle(request)
    }

    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let mut repository = RedisRepository::new(&self.redis_pool)?;
        let list_link = ListLinks::new(&mut repository);
        let mut controller = GrpcListController::new(list_link);
        controller.handle(request)
    }

    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let mut repository = RedisRepository::new(&self.redis_pool)?;
        let create_link = CreateLink::new(&mut repository);
        let mut controller = GrpcCreateController::new(create_link);
        controller.handle(request)
    }
}
