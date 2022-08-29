use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router, Server};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod context;
mod handler;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn hello(state: Extension<StateRef>) -> impl IntoResponse {
    if let Ok(mut state) = state.lock() {
        state.ctr += 1;
        (StatusCode::OK, format!("Hello #{}!\n", state.ctr))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "oh no.\n".into())
    }
}

#[derive(Deserialize, Serialize)]
struct Echo {
    msg: String,
}

#[derive(Deserialize, Serialize)]
struct KeyPair {
    key: String,
    value: Option<String>,
}

async fn json_echo(Json(data): Json<Echo>) -> Json<Echo> {
    println!("echo received \"{}\"", data.msg);
    sleep(Duration::from_millis(500)).await;
    Json(data)
}

async fn get_key(
    Path(key): Path<String>,
    state: Extension<StateRef>,
) -> Result<String, StatusCode> {
    let value;
    {
        let store = &state
            .lock()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .store;
        value = store.get(&key).ok_or(StatusCode::NOT_FOUND)?.clone()
    }
    Ok(value)
}

async fn set_key(
    Path(key): Path<String>,
    body: String,
    state: Extension<StateRef>,
) -> Result<(), StatusCode> {
    {
        state
            .lock()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .store
            .insert(key, body);
    }
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

type StateRef = Arc<Mutex<State>>;

#[tokio::main]
async fn main() {
    let state_ref: StateRef = Arc::new(Mutex::new(State::new()));

    context::wat();
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/json-echo", post(json_echo))
        .route("/kv/:key", get(get_key).post(set_key))
        .layer(Extension(state_ref));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Starting up on http://127.0.0.1:{}", addr.port());
    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}
