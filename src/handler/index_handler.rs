use actix_web::{HttpRequest, HttpResponse};

pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Index")
}
