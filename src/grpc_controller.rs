use r2d2::Pool;
use redis::Client;
use tonic::{Request, Response, Status};

use crate::entity;
use crate::interactor::{CreateLink, GetLink, ListLinks};
use crate::kingsol_api::{CreateRequest, CreateResponse, GetRequest, GetResponse, Link, ListRequest, ListResponse};
use crate::kingsol_api::kingsol_api_server::KingsolApi;
use crate::redis_repository::RedisRepository;
use crate::repository::RepositoryError;
use crate::use_case::{AppError, CreateLinkInput, CreateLinkUseCase, GetLinkInput, GetLinkUseCase, ListLinksInput, ListLinksUseCase};

pub struct API {
    pub redis_pool: Pool<Client>,
}

impl From<RepositoryError> for Status {
    fn from(e: RepositoryError) -> Self {
        Status::internal(e.to_string())
    }
}

impl From<AppError> for Status {
    fn from(e: AppError) -> Self {
        match e {
            AppError::NotFound(s) => Status::not_found(s),
            AppError::AlreadyExists(s) => Status::already_exists(s),
            AppError::ValidationError(s) => Status::invalid_argument(s),
            _ => Status::internal(e.to_string())
        }
    }
}

impl From<entity::Link> for Link {
    fn from(l: entity::Link) -> Self {
        Link { key: l.key, uri: l.uri }
    }
}

impl From<Link> for entity::Link {
    fn from(l: Link) -> Self {
        entity::Link { key: l.key, uri: l.uri }
    }
}

#[tonic::async_trait]
impl KingsolApi for API {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let mut repository = RedisRepository::new(&self.redis_pool)?;
        let mut get_link = GetLink::new(&mut repository);
        let input = GetLinkInput {
            key: request.into_inner().key
        };
        let output = get_link.handle(input)?;

        Ok(Response::new(GetResponse {
            link: Some(Link::from(output.link))
        }))
    }

    async fn list(&self, _request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let mut repository = RedisRepository::new(&self.redis_pool)?;
        let mut list_link = ListLinks::new(&mut repository);
        let output = list_link.handle(ListLinksInput {})?;

        Ok(Response::new(ListResponse {
            links: output.links.into_iter().map(|l| Link::from(l)).collect()
        }))
    }

    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        let mut repository = RedisRepository::new(&self.redis_pool)?;
        let mut create_link = CreateLink::new(&mut repository);
        let link = request.get_ref().link.as_ref().ok_or(Status::invalid_argument("empty link"))?;
        let input = CreateLinkInput {
            overwrite: request.get_ref().overwrite,
            link: entity::Link::from(link.clone()),
        };
        let _ = create_link.handle(input)?;
        Ok(Response::new(CreateResponse {}))
    }
}
