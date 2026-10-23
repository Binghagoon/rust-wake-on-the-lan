use crate::magic_packet::send_magic_packet;
use serde::Serialize;

use std::process;

use axum::{
    extract::State,
    http::Request,
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
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    // Nginx가 전달해 준 X-Forwarded-For 또는 X-Real-IP에서 실제 클라이언트 IP 추출
                    let client_ip = request
                        .headers()
                        .get("x-forwarded-for")
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
                        .or_else(|| {
                            request
                                .headers()
                                .get("x-real-ip")
                                .and_then(|v| v.to_str().ok())
                                .map(|s| s.to_string())
                        })
                        .unwrap_or_else(|| "unknown".to_string());

                    tracing::info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                        client_ip = %client_ip,
                    )
                }),
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
