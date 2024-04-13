#[macro_use]
extern crate diesel;

mod db;
mod models;
mod schema;

use std::usize;

use axum::{
    http::StatusCode,
    response::Redirect,
    routing::{delete, get, post, put},
    Router,
};
use db::estabilish_connection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use models::NewShortLink;
use serde::Deserialize;

use crate::models::Link;

static URL_ID_MAX_CHARS: usize = 7;

#[derive(Debug)]
enum SolveError {
    UrlNotFound,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/r/:id", get(redirect_handler))
        .route("/api/create", post(create_link_handler))
        .route("/api/edit", put(update_link_handler))
        .route("/api/delete", delete(delete_link_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
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

#[derive(Debug, Deserialize)]
struct CreateLinkJson {
    long_url: String,
}

#[derive(Debug, Deserialize)]
struct UpdateLinkJson {
    id: i32,
    long_url: String,
}

#[derive(Debug, Deserialize)]
struct DeleteLinkJson {
    id: i32,
}

async fn create_link_handler(data: axum::extract::Json<CreateLinkJson>) -> StatusCode {
    use crate::schema::links::dsl::*;

    let mut connection = estabilish_connection();
    let new_link = NewShortLink {
        // TODO: Auto generate url_id
        url_id: &generate_url_id().await,
        long_url: &data.long_url,
    };

    diesel::insert_into(links)
        .values(&new_link)
        .execute(&mut connection)
        .expect("Error creating new link");

    StatusCode::OK
}

async fn update_link_handler(data: axum::extract::Json<UpdateLinkJson>) -> StatusCode {
    use crate::schema::links::dsl::*;

    let mut connection = estabilish_connection();
    let db_link = Link {
        id: data.id,
        // TODO: Get the url_id from the database
        url_id: "ffaf".to_string(),
        long_url: data.long_url.to_string(),
    };

    diesel::update(links.find(data.id))
        .set(&db_link)
        .execute(&mut connection)
        .expect("Error updanting link");

    StatusCode::OK
}

async fn delete_link_handler(data: axum::extract::Json<DeleteLinkJson>) -> StatusCode {
    use crate::schema::links::dsl::*;

    let mut connection = estabilish_connection();
    diesel::delete(links.find(data.id))
        .execute(&mut connection)
        .expect("Error deleting link");

    StatusCode::OK
}

async fn get_counter() -> i64 {
    use crate::schema::links::dsl::*;

    let mut connection = estabilish_connection();
    let last_counter = links
        .order(id.desc())
        .select(id)
        .first::<i32>(&mut connection);

    match last_counter {
        Ok(last_counter) => (last_counter as i64) + 100000000000,
        _ => 100000000000,
    }
}

async fn generate_url_id() -> String {
    let mut counter: i64 = get_counter().await;
    let elements: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let mut string_buffer = String::new();
    while counter != 0 {
        string_buffer.insert(0, elements[(counter % 62) as usize]);
        counter /= 62;
    }
    while string_buffer.len() != URL_ID_MAX_CHARS {
        string_buffer.insert(0, '0');
    }
    string_buffer.to_string()
}

async fn solve_id(received_url_id: String) -> Result<String, SolveError> {
    use crate::schema::links::dsl::*;

    let mut connection = estabilish_connection();

    let result = links
        .filter(url_id.eq(received_url_id))
        .select(long_url)
        .first::<String>(&mut connection);

    match result {
        Ok(data) => Ok(data),
        _ => Err(SolveError::UrlNotFound),
    }
}
