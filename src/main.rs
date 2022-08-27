use axum::http::status::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router, Server};
use serde::{Deserialize, Serialize};

use std::net::SocketAddr;

mod context;
mod handler;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, "Well this is easy.\n")
}

#[derive(Deserialize, Serialize)]
struct Echo {
    msg: String,
}

async fn json_echo(Json(data): Json<Echo>) -> Json<Echo> {
    println!("echo received \"{}\"", data.msg);
    Json(data)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/json-echo", post(json_echo))
        .route("/kv/:key", get(hello));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Starting up on http://{:?}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}
