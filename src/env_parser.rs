use std::{
    env::{self, VarError},
    net::{IpAddr, Ipv4Addr},
    os::windows::process,
};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub mac_address: String,
    pub server_address: IpAddr,
    pub port: u16,
}
pub fn get_env() -> ServerConfig {
    let mac_address: String = get_mac_address();
    let port: u16 = get_port();
    let server_address: IpAddr = get_server_address();
    println!("- MAC Address: {}", mac_address);
    println!("- Server: http://{}:{}", server_address, port);
    // I want to return {macAddress, address, port} as in Typescript
    ServerConfig {
        mac_address,
        server_address,
        port,
    }
}
fn get_mac_address() -> String {
    let mac_address = env::var("MAC_ADDRESS").expect("MAC_ADDRESS environment variable not set");
    
    let mac_regex = Regex::new(r"^([0-9A-Fa-f]{2}[:\-]){5}([0-9A-Fa-f]{2})$")
        .expect("Failed to compile MAC address regex");

    if !mac_regex.is_match(&mac_address) {
        eprintln!("MAC_ADDRESS environment variable must be in format XX:XX:XX:XX:XX:XX");
        std::process::exit(1);
    }

    mac_address
}
fn get_port() -> u16 {
    const PORT_KEY: &str = "PORT";
    const DEFAULT_PORT: &str = "3000";
    env::var(PORT_KEY)
        .unwrap_or_else(|_| {
            eprintln!(
                "{} environment variable not set, defaulting to {}",
                PORT_KEY, DEFAULT_PORT
            );
            DEFAULT_PORT.to_string()
        })
        .parse::<u16>()
        .unwrap_or_else(|_| {
            eprintln!(
                "{} environment variable must be a valid port number, defaulting to {}",
                PORT_KEY, DEFAULT_PORT
            );
            DEFAULT_PORT
                .parse::<u16>()
                .expect("Failed to parse default port")
        })
}

fn get_server_address() -> IpAddr {
    const SERVER_ADDRESS_KEY: &str = "SERVER_ADDRESS";
    let default_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    env::var(SERVER_ADDRESS_KEY)
        .map_or_else(
            |e| {
                eprintln!(
                    "{} environment variable not set, defaulting to {}",
                    SERVER_ADDRESS_KEY, default_addr
                );
                Ok(default_addr)
            },
            |s| s.parse::<IpAddr>(),
        )
        .unwrap_or_else(|e| {
            eprintln!(
                "{} is not a valid IP address. Defaulting to {}",
                SERVER_ADDRESS_KEY, default_addr
            );
            default_addr
        })
}
