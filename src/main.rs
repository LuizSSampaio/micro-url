#[macro_use]
extern crate diesel;

mod db;
mod schema;
mod models;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug)]
enum SolveError {
    UrlNotFound,
}

#[get("/{url_id}")]
async fn redirect(path: web::Path<String>) -> impl Responder {
    match solve_id(path.into_inner()).await {
        Ok(url) => HttpResponse::Found().append_header(("location", url)).finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn solve_id(url_id: String) -> Result<String, SolveError> {
    if url_id == "ffff" {
        return Ok("https://youtube.com".to_string())
    }
    Err(SolveError::UrlNotFound)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(redirect)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
