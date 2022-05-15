use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::header;
use r2d2::Pool;
use redis::Client;

use crate::interactor::GetLink;
use crate::redis_repository::RedisRepository;
use crate::repository::RepositoryError;
use crate::use_case::{AppError, GetLinkInput, GetLinkUseCase};

pub async fn link_handler(req: HttpRequest, pool: web::Data<Pool<Client>>) -> HttpResponse {
    let repository = RedisRepository::new(pool.get_ref());
    if let Err(e) = repository {
        return match e {
            RepositoryError::NotFound(_) => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish()
        };
    }

    let mut get_link = GetLink::<RedisRepository>::new(repository.unwrap());
    let input = GetLinkInput {
        key: (&req.path()[1..]).to_string()
    };
    let output = get_link.handle(input);
    if let Err(e) = output {
        return match e {
            AppError::NotFound(_) => HttpResponse::NotFound().finish(),
            AppError::ValidationError(_) => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish()
        };
    }
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, output.unwrap().link.uri))
        .finish()
}
