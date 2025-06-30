use axum::{Router, routing::get};
use std::net::SocketAddr;

mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    // Create app and define routes
    let app = routes::create_router();

    // Server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server running at http://{}", addr);

    // ✅ Native axum v0.7 server syntax — no hyper import needed
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}