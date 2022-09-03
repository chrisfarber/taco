use axum::extract::Path;
use axum::http::status::StatusCode;

use axum::routing::{get, post};
use axum::{Extension, Json, Router, Server};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tokio::time::sleep;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

mod db;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn hello(state_ref: Extension<StateRef>) -> Result<String, StatusCode> {
    let mut state = state_ref.write().await;
    state.ctr += 1;
    Ok(format!("Hello #{}!\n", state.ctr))
}

#[derive(Deserialize, Serialize)]
struct Echo {
    msg: String,
}

async fn json_echo(Json(data): Json<Echo>) -> Json<Echo> {
    println!("echo received \"{}\"", data.msg);
    sleep(Duration::from_millis(500)).await;
    Json(data)
}

async fn get_key(
    Path(key): Path<String>,
    state: Extension<db::Database>,
) -> Result<String, StatusCode> {
    db::get_key(&state, key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
}

async fn set_key(
    Path(key): Path<String>,
    body: String,
    state: Extension<db::Database>,
) -> Result<(), StatusCode> {
    Ok(())
}

struct State {
    pub ctr: i32,
    pub store: HashMap<String, String>,
}

impl State {
    fn new() -> State {
        State {
            ctr: 0,
            store: HashMap::new(),
        }
    }
}

type StateRef = Arc<RwLock<State>>;

#[tokio::main]
async fn main() {
    // let state_ref: StateRef = Arc::new(Mutex::new(State::new()));
    let state_ref: StateRef = Arc::new(RwLock::new(State::new()));

    let db_opts = db::DbOptions::new("sqlite://db.sqlite");
    db::init_db_if_needed(&db_opts)
        .await
        .expect("error initializing db");

    let db = db::open_pool(&db_opts)
        .await
        .expect("Could not connect to sqlite db");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tower_http=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/json-echo", post(json_echo))
        .route("/kv/:key", get(get_key).post(set_key))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(state_ref))
                .layer(Extension(db))
                .layer(TraceLayer::new_for_http()),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Starting up on http://127.0.0.1:{}", addr.port());
    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}
