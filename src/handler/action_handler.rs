use actix_web::{get, HttpResponse, web};
use actix_web::http::header;
use anyhow::Result;
use r2d2::Pool;
use redis::{Client, Commands};
use serde::Deserialize;

use crate::redis::{get_connection, LINK_KEY_PREFIX};

fn do_add(query: web::Query<AddQuery>, pool: web::Data<Pool<Client>>) -> Result<HttpResponse> {
    let key = query.key.clone();
    let value = query.value.clone();
    if key.is_empty() || key == "!" || value.is_empty() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let mut conn = get_connection(&pool)?;
    conn.set(format!("{}:{}", LINK_KEY_PREFIX, key), value)?;
    Ok(HttpResponse::TemporaryRedirect().append_header((header::LOCATION, "/!/list")).finish())
}

#[derive(Deserialize)]
pub struct AddQuery {
    key: String,
    value: String,
}

#[get("/!/add")]
pub async fn add(query: web::Query<AddQuery>, pool: web::Data<Pool<Client>>) -> HttpResponse {
    match do_add(query, pool) {
        Ok(r) => r,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

fn do_list(pool: web::Data<Pool<Client>>) -> Result<HttpResponse> {
    let mut conn = get_connection(&pool)?;
    let keys: Vec<String> = conn.keys(format!("{}:*", LINK_KEY_PREFIX))?;
    let body = keys
        .into_iter()
        .map(|k| format!("{} -> {}", k.clone(), conn.get(k).unwrap_or("?".to_string())))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(HttpResponse::Ok().body(body))
}

#[get("/!/list")]
pub async fn list(pool: web::Data<Pool<Client>>) -> HttpResponse {
    match do_list(pool) {
        Ok(r) => r,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}
