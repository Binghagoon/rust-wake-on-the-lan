use std::net::UdpSocket;

pub fn send_magic_packet(mac_address: &str) -> Result<(), String> {
    let mac_bytes = parse_mac_address(mac_address)?;
    let mut packet = vec![0xFF; 6];

    for _ in 0..16 {
        packet.extend_from_slice(&mac_bytes);
    }

    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|error| error.to_string())?;
    socket
        .set_broadcast(true)
        .map_err(|error| error.to_string())?;
    socket
        .send_to(&packet, "255.255.255.255:9")
        .map_err(|error| error.to_string())?;

    Ok(())
}

fn parse_mac_address(mac_address: &str) -> Result<[u8; 6], String> {
    let normalized_mac: String = mac_address
        .chars()
        .filter(|character| character.is_ascii_hexdigit())
        .collect();

    if normalized_mac.len() != 12 {
        return Err("MAC address must contain exactly 12 hexadecimal digits".to_string());
    }

    let mut bytes = [0u8; 6];

    for (index, byte) in bytes.iter_mut().enumerate() {
        let start = index * 2;
        let end = start + 2;
        *byte = u8::from_str_radix(&normalized_mac[start..end], 16)
            .map_err(|error| error.to_string())?;
    }

    Ok(bytes)
}
