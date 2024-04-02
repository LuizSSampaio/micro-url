#[macro_use]
extern crate diesel;

mod db;
mod schema;
mod models;

use axum::{response::Redirect, routing::get, Router};

#[derive(Debug)]
enum SolveError {
    UrlNotFound,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/r/:id", get(redirect_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn redirect_handler(params: axum::extract::Path<String>) -> Redirect {
    let id = params.to_string();

    match solve_id(id).await {
        Ok(url) => Redirect::temporary(&url),
        // TODO: CHANGE THIS URL TO 404 PAGE
        Err(_) => Redirect::temporary("https://http.cat/404"),
    }
}

async fn solve_id(id: String) -> Result<String, SolveError> {
    if id == "ffff" {
        return Ok("https://youtube.com".to_string())
    }
    Err(SolveError::UrlNotFound)
}
