use axum::extract::Extension;
use axum::handler::get;
use axum::{AddExtensionLayer, Router};
use hyper::{HeaderMap, Server, StatusCode};
use std::net::SocketAddr;
use std::sync::{atomic::AtomicUsize, Arc};

#[derive(Debug, Default, Clone)]
struct AppState {
    counter: Arc<AtomicUsize>,
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new()
        .route("/", get(home))
        .layer(AddExtensionLayer::new(AppState::default()));

    let server = Server::bind(&addr).serve(app.into_make_service());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn home(state: Extension<AppState>) -> (StatusCode, HeaderMap, String) {
    let counter = state
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/plain; charset=utf-8".parse().unwrap());
    let body = format!("Counter is at: {}", counter);
    (StatusCode::OK, headers, body)
}
