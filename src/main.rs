mod config;
mod error;
mod routes;
mod handlers;
mod types;
mod utils;

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
use std::env;

#[tokio::main]
async fn main() {
    // Initialize logging
    println!("Starting Solana HTTP Server...");

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    // Build the application with all routes
    let app = Router::new()
        .merge(routes::health_routes())
        .merge(routes::keypair_routes())
        .merge(routes::token_routes())
        .merge(routes::message_routes())
        .merge(routes::send_routes())
        .layer(cors);

    // Get port from environment or default to 3000
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Server listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}