use tonic::{Request, Response, Status};

use crate::entity;
use crate::kingsol_api::{
    CreateRequest, CreateResponse, GetRequest, GetResponse, Link, ListRequest, ListResponse,
};
use crate::repository::RepositoryError;
use crate::use_case::{
    AppError, CreateLinkInput, CreateLinkUseCase, GetLinkInput, GetLinkUseCase, ListLinksInput,
    ListLinksUseCase,
};

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
            _ => Status::internal(e.to_string()),
        }
    }
}

impl From<entity::Link> for Link {
    fn from(l: entity::Link) -> Self {
        Link {
            key: l.key,
            uri: l.uri,
        }
    }
}

impl From<Link> for entity::Link {
    fn from(l: Link) -> Self {
        entity::Link {
            key: l.key,
            uri: l.uri,
        }
    }
}

pub struct GrpcGetController<U: GetLinkUseCase> {
    get_link: U,
}

impl<U: GetLinkUseCase> GrpcGetController<U> {
    pub fn new(get_link: U) -> Self {
        Self { get_link }
    }

    pub fn handle(
        &mut self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetResponse>, Status> {
        let input = GetLinkInput {
            key: request.into_inner().key,
        };
        let output = self.get_link.handle(input)?;

        Ok(Response::new(GetResponse {
            link: Some(Link::from(output.link)),
        }))
    }
}

pub struct GrpcListController<U: ListLinksUseCase> {
    list_link: U,
}

impl<U: ListLinksUseCase> GrpcListController<U> {
    pub fn new(list_link: U) -> Self {
        Self { list_link }
    }

    pub fn handle(
        &mut self,
        _request: Request<ListRequest>,
    ) -> Result<Response<ListResponse>, Status> {
        let output = self.list_link.handle(ListLinksInput {})?;

        Ok(Response::new(ListResponse {
            links: output.links.into_iter().map(|l| Link::from(l)).collect(),
        }))
    }
}

pub struct GrpcCreateController<U: CreateLinkUseCase> {
    create_link: U,
}

impl<U: CreateLinkUseCase> GrpcCreateController<U> {
    pub fn new(create_link: U) -> Self {
        Self { create_link }
    }

    pub fn handle(
        &mut self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let link = request
            .get_ref()
            .link
            .as_ref()
            .ok_or(Status::invalid_argument("empty link"))?;
        let input = CreateLinkInput {
            overwrite: request.get_ref().overwrite,
            link: entity::Link::from(link.clone()),
        };
        let _ = self.create_link.handle(input)?;
        Ok(Response::new(CreateResponse {}))
    }
}
