use tonic::{Request, Response, Status};
use crate::proto::kingsol::{CreateRequest, CreateResponse, GetRequest, GetResponse, ListRequest, ListResponse};
use crate::proto::kingsol::kingsol_api_server::KingsolApi;

#[derive(Default)]
pub struct API {}

#[tonic::async_trait]
impl KingsolApi for API {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        todo!()
    }

    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        todo!()
    }
}
