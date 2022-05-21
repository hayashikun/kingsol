use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::header;
use r2d2::Pool;
use redis::Client;

use crate::interactor::GetLink;
use crate::redis_repository::RedisRepository;
use crate::use_case::{AppError, GetLinkInput, GetLinkUseCase};

pub async fn link_handler(req: HttpRequest, pool: web::Data<Pool<Client>>) -> HttpResponse {
    let repository = RedisRepository::new(pool.get_ref());
    if let Err(_) = repository {
        return HttpResponse::InternalServerError().finish();
    }
    let mut repository = repository.unwrap();

    let mut get_link = GetLink::<RedisRepository>::new(&mut repository);
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
