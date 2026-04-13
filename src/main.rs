use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, process};

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
    let mac_address = std::env::var("MAC_ADDRESS").unwrap_or_else(|_| {
        eprintln!("MAC_ADDRESS environment variable not set");
        process::exit(1);
    });
    println!("MAC Address: {}", mac_address);

    // 주소 바인딩
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

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
