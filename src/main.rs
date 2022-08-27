use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

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
    (axum::http::status::StatusCode::OK, "Well this is easier.\n")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello));
    // let service = ServiceBuilder::new().service_fn(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Starting up on http://{:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}
