mod env_parser;
mod route;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::{
    net::{IpAddr, SocketAddr},
    process,
};
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
    let listener = tokio::net::TcpListener::bind(SocketAddr::from((server_address, port)))
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Socket is not valid or already in use, address: http://{server_address}:{port}",
            );
        });

    route(listener, mac_address, server_address, port).await;
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
