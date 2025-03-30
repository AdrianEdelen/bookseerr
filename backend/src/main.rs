use axum::{
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber;
use tokio::net::TcpListener;


#[derive(Debug, Serialize, Deserialize)]
struct BookRequest {
    title: String,
    author: String,
}

async fn healthcheck() -> &'static str {
    "OK"
}

async fn list_requests() -> Json<Vec<BookRequest>> {
    Json(vec![
        BookRequest {
            title: "Dune".into(),
            author: "Frank Herbert".into(),
        },
        BookRequest {
            title: "The Hobbit".into(),
            author: "J.R.R. Tolkien".into(),
        },
    ])
}

async fn create_request(Json(payload): Json<BookRequest>) -> Json<BookRequest> {
    println!("Received request: {:?}", payload);
    Json(payload)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(healthcheck))
        .route("/requests", get(list_requests).post(create_request));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
 
}