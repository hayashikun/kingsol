use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    format!("Index")
}


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Running on http://localhost:{}", port);
    HttpServer::new(|| {
        App::new().service(index).service(greet)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
