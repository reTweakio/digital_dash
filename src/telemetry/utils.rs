use local_ip_address::local_ip;
use std::net::UdpSocket;

pub fn setup_udp_socket() -> UdpSocket {
    let ip_addr: String = local_ip().unwrap().to_string();
    let port: &str = "8080";
    let binding_addr: String = format!("{}:{}", ip_addr, port);
    let socket: UdpSocket = UdpSocket::bind(binding_addr).unwrap();
    socket
}

pub fn parse_f32_from_bytes(buf: &[u8]) -> f32 {
    f32::from_le_bytes(buf.try_into().unwrap())
}

pub fn parse_i16_from_bytes(buf: &[u8]) -> i16 {
    i16::from_le_bytes(buf.try_into().unwrap())
}
