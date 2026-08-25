mod env_parser;
mod magic_packet;
mod route;

use crate::route::route;
use std::net::SocketAddr;
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

    route(listener, mac_address).await;
}
