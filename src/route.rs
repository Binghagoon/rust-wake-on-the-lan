use std::{net::IpAddr, process};

use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use tokio::net::TcpListener;

use crate::magic_packet::send_magic_packet;

#[derive(Clone)]
struct AppState {
    mac_address: String,
    server_address: IpAddr,
    port: u16,
}

pub async fn route(
    listener: TcpListener,
    mac_address: String,
    server_address: IpAddr,
    port: u16,
) -> () {
    let state = AppState {
        mac_address,
        server_address,
        port,
    };
    axum::serve(
        listener,
        Router::new()
            .route("/echo", post(post_echo))
            .route("/", get(get_index))
            .with_state(state),
    )
    .await
    .and_then(|| {
        println!("Server started");
        ()
    })
    .unwrap_or_else(|e| {
        eprintln!("Failed to start server: {}", e);
        process::exit(1);
    })
}
async fn get_index() -> Html<&'static str> {
    Html(include_str!("../public/index.html"))
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
                mac_address: state.mac_address.clone(),
                error: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(EchoResponse {
                message: body,
                magic_packet_sent: false,
                mac_address: state.mac_address.clone(),
                error: Some(e.to_string()),
            }),
        ),
    }
}
