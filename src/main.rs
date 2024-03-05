#![allow(unused)]  // For beginning


use axum::extract::{Path, Query};
use axum::handler::HandlerWithoutStateExt;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::{get, get_service};
use tokio::net::TcpListener;
use serde::Deserialize;
use tower_http::services::ServeDir;

mod error;
mod error;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// ex) `/hello?name=Vince`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("default world");

    Html(format!("Hello <strong>{name}</strong>"))
}

// ex. `/hello2/Vince
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
