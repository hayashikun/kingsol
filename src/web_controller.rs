use actix_web::http::header;
use actix_web::{HttpRequest, HttpResponse};

use crate::use_case::{AppError, GetLinkInput, GetLinkUseCase};

pub struct LinkController<U: GetLinkUseCase> {
    get_link: U,
}

impl<U: GetLinkUseCase> LinkController<U> {
    pub fn new(get_link: U) -> Self {
        Self { get_link }
    }

    pub async fn handle(&mut self, req: HttpRequest) -> HttpResponse {
        let input = GetLinkInput {
            key: (&req.path()[1..]).to_string(),
        };
        let output = self.get_link.handle(input).await;

        if let Err(e) = output {
            return match e {
                AppError::NotFound(_) => HttpResponse::NotFound().finish(),
                AppError::ValidationError(_) => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            };
        }
        HttpResponse::TemporaryRedirect()
            .append_header((header::LOCATION, output.unwrap().link.uri))
            .finish()
    }
}
