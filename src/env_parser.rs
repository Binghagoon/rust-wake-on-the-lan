use std::env;

pub fn exit_on_missing_env_var(str: &str) -> ! {
    eprintln!("{} environment variable not set", str);
    process::exit(1)
}
pub fn get_env() -> (String, String, u16) {
    let mac_address: String = get_mac_address();
    let port: u16 = get_port();
    let address: String = get_server_address();
    println!("- MAC Address: {}", mac_address);
    println!("- Server: http://{}:{}", address, port);

    (mac_address, address, port)
}
fn get_mac_address() -> String {
    let mac_address = env::var("MAC_ADDRESS");
    let mac_address = mac_address.unwrap_or_else(|| exit_on_missing_env_var("MAC_ADDRESS"));
    let mac_regex = regex::Regex::new(r"^([0-9A-Fa-f]{2}[:\-]){5}([0-9A-Fa-f]{2})$").unwrap();
    if !mac_regex.is_match(&mac_address) {
        eprintln!("MAC_ADDRESS environment variable must be in format XX:XX:XX:XX:XX:XX");
        process::exit(1)
    }
    mac_address
}
fn get_port() -> u16 {
    const PORT_KEY: &str = "PORT";
    env::var(PORT_KEY)
        // 1. 값이 없으면 기본값 "3000"을 사용하고 로그 출력
        .unwrap_or_else(|_| {
            eprintln!(
                "{} environment variable not set, defaulting to 3000",
                PORT_KEY
            );
            "3000".to_string()
        })
        // 2. String을 u16으로 파싱
        .parse::<u16>()
        // 3. 파싱 실패 시 에러 로그와 함께 프로세스 종료
        .unwrap_or_else(|_| {
            eprintln!("{} must be a valid u16 number", PORT_KEY);
            process::exit(1)
        })
}

fn get_server_address() -> String {
    const SERVER_ADDRESS_KEY: &str = "SERVER_ADDRESS";

    env::var(SERVER_ADDRESS_KEY).unwrap_or_else(|| {
        eprintln!(
            "{} environment variable not set, defaulting to localhost",
            SERVER_ADDRESS_KEY
        );
        "localhost".to_string()
    })
}
