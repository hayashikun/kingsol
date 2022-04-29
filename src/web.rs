use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::header;
use anyhow::Result;
use r2d2::Pool;
use redis::{Client, Commands};

use crate::redis::{get_connection, LINK_KEY_PREFIX};

fn do_link(req: HttpRequest, pool: web::Data<Pool<Client>>) -> Result<HttpResponse> {
    let mut conn = get_connection(&pool)?;
    let key = &req.path()[1..];
    let loc: Option<String> = conn.get(format!("{}:{}", LINK_KEY_PREFIX, key))?;
    if let Some(loc) = loc {
        Ok(HttpResponse::TemporaryRedirect().append_header((header::LOCATION, loc)).finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn link_handler(req: HttpRequest, pool: web::Data<Pool<Client>>) -> HttpResponse {
    match do_link(req, pool) {
        Ok(r) => r,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}
