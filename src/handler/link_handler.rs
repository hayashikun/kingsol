use actix_web::{HttpRequest, HttpResponse};

pub async fn link(req: HttpRequest) -> HttpResponse {
    let path = req.path();
    println!("{path:?}");
    HttpResponse::Ok().body(path.to_string())
}
