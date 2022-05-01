use r2d2::Pool;
use redis::{Client, Commands};
use tonic::{Request, Response, Status};

use crate::kingsol_api::{CreateRequest, CreateResponse, GetRequest, GetResponse, Link, ListRequest, ListResponse};
use crate::kingsol_api::kingsol_api_server::KingsolApi;
use crate::redis::{get_connection, LINK_KEY_PREFIX};

pub struct API {
    pub redis_pool: Pool<Client>,
}

#[tonic::async_trait]
impl KingsolApi for API {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let key = request.into_inner().key;

        let mut conn = get_connection(&self.redis_pool)
            .map_err(|e| Status::internal(e.to_string()))?;
        let uri: Option<String> = conn.get(format!("{}:{}", LINK_KEY_PREFIX, key))
            .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(uri) = uri {
            Ok(Response::new(GetResponse {
                link: Some(Link { key, uri })
            }))
        } else {
            Err(Status::not_found("link is not registered"))
        }
    }

    async fn list(&self, _request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let mut conn = get_connection(&self.redis_pool)
            .map_err(|e| Status::internal(e.to_string()))?;
        let keys: Vec<String> = conn.keys(format!("{}:*", LINK_KEY_PREFIX))
            .map_err(|e| Status::internal(e.to_string()))?;
        let links = keys
            .into_iter()
            .map(|k| (k.clone(), conn.get(k).unwrap_or("".to_string())))
            .map(|(key, uri)| Link { key, uri })
            .collect();
        Ok(Response::new(ListResponse { links }))
    }

    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        let overwrite = request.get_ref().overwrite;
        let link: Link = request.into_inner().link
            .ok_or(Status::invalid_argument("empty key or uri"))?;
        if link.key.is_empty() || link.uri.is_empty() {
            return Err(Status::invalid_argument("empty key or uri"));
        }
        let mut conn = get_connection(&self.redis_pool)
            .map_err(|e| Status::internal(e.to_string()))?;
        if !overwrite {
            let exists = conn.exists(format!("{}:{}", LINK_KEY_PREFIX, link.key))
                .map_err(|e| Status::internal(e.to_string()))?;
            if exists {
                return Err(Status::already_exists(format!("{} exists", link.key)));
            }
        }
        conn.set(format!("{}:{}", LINK_KEY_PREFIX, link.key), link.uri)
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateResponse {}))
    }
}
