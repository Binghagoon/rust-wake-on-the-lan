use crate::magic_packet::send_magic_packet;
use serde::Serialize;
use std::process;

use axum::{
    extract::State,
    http::{HeaderMap, Method, Request, StatusCode, Version},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
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

/// HTTP 요청 헤더에서 실제 클라이언트 IP를 추출하는 헬퍼 함수
fn extract_client_ip(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

pub async fn route(listener: TcpListener, mac_address: String) {
    let state = AppState { mac_address };

    let app = Router::new()
        .route("/wake", post(post_wake))
        .route("/", get(get_index))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let client_ip = extract_client_ip(request.headers());

                    tracing::info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                        client_ip = %client_ip,
                    )
                })
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

    println!("Server started");

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to start server: {}", e);
            process::exit(1);
        });
}

async fn get_index() -> Html<&'static str> {
    Html(include_str!("../public/index.html"))
}

async fn post_wake(
    State(state): State<AppState>,
    body: String,
) -> (StatusCode, Json<EchoResponse>) {
    let result = send_magic_packet(&state.mac_address);
    
    let (status, magic_packet_sent, error) = match result {
        Ok(()) => (StatusCode::OK, true, None),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, false, Some(e.to_string())),
    };

    (
        status,
        Json(EchoResponse {
            message: body,
            magic_packet_sent,
            mac_address: state.mac_address.clone(),
            error,
        }),
    )
}