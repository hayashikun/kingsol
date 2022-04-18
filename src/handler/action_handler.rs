use actix_web::{get, post, HttpResponse, web};
use actix_web::http::header;
use anyhow::Result;
use r2d2::Pool;
use redis::{Client, Commands};
use serde::Deserialize;

use crate::redis::{get_connection, LINK_KEY_PREFIX};

fn do_create(params: web::Form<CreateParams>, pool: web::Data<Pool<Client>>) -> Result<HttpResponse> {
    let key = params.key.clone();
    let url = params.url.clone();
    if key.is_empty() || key == "!" || url.is_empty() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let mut conn = get_connection(&pool)?;
    conn.set(format!("{}:{}", LINK_KEY_PREFIX, key), url)?;
    Ok(HttpResponse::SeeOther().append_header((header::LOCATION, "/!/list")).finish())
}

#[derive(Deserialize)]
pub struct CreateParams {
    key: String,
    url: String,
}

#[post("/!/create")]
pub async fn create(params: web::Form<CreateParams>, pool: web::Data<Pool<Client>>) -> HttpResponse {
    match do_create(params, pool) {
        Ok(r) => r,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

fn do_list(pool: web::Data<Pool<Client>>) -> Result<HttpResponse> {
    let mut conn = get_connection(&pool)?;
    let keys: Vec<String> = conn.keys(format!("{}:*", LINK_KEY_PREFIX))?;

    let mut html = "<html><body>".to_string();
    html += "<ul>";
    html += &*keys
        .into_iter()
        .map(|k| format!("<li>{} -> {}</li>", k.clone(), conn.get(k).unwrap_or("?".to_string())))
        .collect::<Vec<_>>()
        .join("");
    html += "</ul>";
    html += "
    <form class='form-horizontal' method='POST' action='/!/create'>
        <div class='form-group'>
            <label for='key'>Key</label>
            <input type='text' name='key' id='key' class='form-control'>
            <label for='value'>Value</label>
            <input type='text' name='url' id='url' class='form-control'>
            <button type='submit'>Add</button>
        </div>
    </form>";
    html += "</body></html>";

    Ok(HttpResponse::Ok().body(html))
}

#[get("/!/list")]
pub async fn list(pool: web::Data<Pool<Client>>) -> HttpResponse {
    match do_list(pool) {
        Ok(r) => r,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}
