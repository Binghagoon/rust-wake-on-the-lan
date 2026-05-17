use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    net::{SocketAddr, TcpListener},
    process,
};
mod env_parser;
// I want to create a simple REST API with axum that has two endpoints: GET /hello and POST /echo.
// The GET /hello endpoint should return a JSON response with a greeting message,
// and the POST /echo endpoint should accept a string payload and return it back in the response.

#[tokio::main]
async fn main() {
    // 라우터 설정
    let env_parser::ServerConfig {
        mac_address,
        server_address,
        port,
    } = env_parser::get_env();
    let listener = tokio::net::TcpListener::bind(SocketAddr::from((server_address, port)))
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Socket is not valid or already in use, address: http://{server_address}:{port}",
            );
        });
    axum::serve(
        listener,
        Router::new().route("/hello", get(get_hello).post(post_echo)),
    )
    .await
    .unwrap_or_else(|e| {
        eprintln!("Failed to start server: {}", e);
        process::exit(1);
    });
}

async fn get_hello() -> Json<Greeting> {
    Json(Greeting {
        message: "Hello, World!".to_string(),
    })
}

async fn post_echo(){

}