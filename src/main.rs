mod env_parser;
mod magic_packet;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use magic_packet::send_magic_packet;
use serde::Serialize;
use std::{
    net::{IpAddr, SocketAddr},
    process,
};
#[derive(Clone)]
struct AppState {
    mac_address: String,
    server_address: IpAddr,
    port: u16,
}

#[derive(Serialize)]
struct Greeting {
    message: String,
    mac_address: String,
    server_address: String,
    port: u16,
}

#[derive(Serialize)]
struct EchoResponse {
    message: String,
    magic_packet_sent: bool,
    mac_address: String,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    // 라우터 설정
    let env_parser::ServerConfig {
        mac_address,
        server_address,
        port,
    } = env_parser::get_env();
    let state = AppState {
        mac_address,
        server_address,
        port,
    };
    let listener = tokio::net::TcpListener::bind(SocketAddr::from((server_address, port)))
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Socket is not valid or already in use, address: http://{server_address}:{port}",
            );
        });

    println!("Server is running at http://{}:{}", server_address, port);

    axum::serve(
        listener,
        Router::new()
            .route("/hello", get(get_hello))
            .route("/echo", post(post_echo))
            .with_state(state),
    )
    .await
    .unwrap_or_else(|e| {
        eprintln!("Failed to start server: {}", e);
        process::exit(1);
    })
}

async fn get_hello(State(state): State<AppState>) -> Json<Greeting> {
    Json(Greeting {
        message: "Hello, World!".to_string(),
        mac_address: state.mac_address,
        server_address: state.server_address.to_string(),
        port: state.port,
    })
}

async fn post_echo(
    State(state): State<AppState>,
    body: String,
) -> (StatusCode, Json<EchoResponse>) {
    match send_magic_packet(&state.mac_address) {
        Ok(()) => (
            StatusCode::OK,
            Json(EchoResponse {
                message: body,
                magic_packet_sent: true,
                mac_address: state.mac_address,
                error: None,
            }),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(EchoResponse {
                message: body,
                magic_packet_sent: false,
                mac_address: state.mac_address,
                error: Some(error),
            }),
        ),
    }
}
