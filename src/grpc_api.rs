use sqlx::{MySql, Pool};
use tonic::{Request, Response, Status};

use crate::grpc_controller::{GrpcCreateController, GrpcGetController, GrpcListController};
use crate::interactor::{CreateLink, GetLink, ListLinks};
use crate::kingsol_api::kingsol_api_server::KingsolApi;
use crate::kingsol_api::{
    CreateRequest, CreateResponse, GetRequest, GetResponse, ListRequest, ListResponse,
};
use crate::mysql_repository::MysqlRepository;

pub struct GrpcApi {
    pool: Pool<MySql>,
}

impl GrpcApi {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl KingsolApi for GrpcApi {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let repository = MysqlRepository::new(&self.pool);
        let get_link = GetLink::new(repository);
        let mut controller = GrpcGetController::new(get_link);
        controller.handle(request).await
    }

    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let repository = MysqlRepository::new(&self.pool);
        let list_link = ListLinks::new(repository);
        let mut controller = GrpcListController::new(list_link);
        controller.handle(request).await
    }

    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let repository = MysqlRepository::new(&self.pool);
        let create_link = CreateLink::new(repository);
        let mut controller = GrpcCreateController::new(create_link);
        controller.handle(request).await
    }
}
