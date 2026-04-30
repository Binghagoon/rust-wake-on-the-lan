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

// DTO(Data Transfer Object) 정의
#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[tokio::main]
async fn main() {
    // 라우터 설정
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust API!" }))
        .route("/users", post(create_user));
    // get mac address from process env
    let (mac_address, server_address, port) = env_parser::get_env();
    let addr = SocketAddr::from((server_address.parse::<std::net::IpAddr>().unwrap(), port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 핸들러 함수
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // 자동으로 JSON 응답 생성
    Json(user)
}
