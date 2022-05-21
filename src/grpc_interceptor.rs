use r2d2::Pool;
use redis::Client;
use tonic::service::Interceptor;
use tonic::{Request, Status};

use crate::redis_repository::RedisRepository;
use crate::repository::Repository;

#[derive(Clone)]
pub struct GrpcInterceptor {
    redis_pool: Pool<Client>,
}

impl GrpcInterceptor {
    pub fn new(redis_pool: Pool<Client>) -> Self {
        Self { redis_pool }
    }
}

impl Interceptor for GrpcInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let mut repository = RedisRepository::new(&self.redis_pool)?;
        let token = request
            .metadata()
            .get("token")
            .ok_or(Status::unauthenticated("missing token"))?
            .to_str()
            .map_err(|e| Status::internal(e.to_string()))?;
        let exists = repository.exist_token(token.to_string())?;
        if exists {
            Ok(request)
        } else {
            Err(Status::unauthenticated("invalid token"))
        }
    }
}
