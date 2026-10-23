use crate::magic_packet::send_magic_packet;
use serde::Serialize;

use std::process;

use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[derive(Serialize)]
struct EchoResponse {
    message: String,
    magic_packet_sent: bool,
    mac_address: String,
    error: Option<String>,
}

#[derive(Clone)]
struct AppState {
    mac_address: String,
}

pub async fn route(listener: TcpListener, mac_address: String) -> () {
    let state = AppState { mac_address };
    axum::serve(
        listener,
        Router::new()
            .route("/wake", post(post_wake))
            .route("/", get(get_index))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            )
            .with_state(state),
    )
    .await
    .and_then(|_| {
        println!("Server started");
        Ok(())
    })
    .unwrap_or_else(|e| {
        eprintln!("Failed to start server: {}", e);
        process::exit(1);
    })
}
async fn get_index() -> Html<&'static str> {
    Html(include_str!("../public/index.html"))
}
async fn post_wake(
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
