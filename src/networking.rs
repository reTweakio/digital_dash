use std::net::UdpSocket;
use local_ip_address::local_ip;


struct PacketInfo {
    current_rpm: f32,
    speed: f32,
    best_lap: f32,
    current_lap: f32,
    current_race_time: f32,
    gear: u8
}

fn setup_udp_socket() -> UdpSocket {
    let ip_addr: String = local_ip().unwrap().to_string();
    let port: &str = "8080";
    let binding_addr: String = format!("{}:{}", ip_addr, port);
    let socket: UdpSocket = UdpSocket::bind(binding_addr).expect("Failed to bind to address");
    return socket
}

pub fn parse_packets() {
    let socket: UdpSocket = setup_udp_socket();

    loop {
        let mut buf: Vec<u8> = vec![0; 1024];
        socket.recv_from(&mut buf).expect("Failed to receive data");

        let packet_info = PacketInfo {
            current_rpm: f32::from_le_bytes(buf[16..20].try_into().unwrap()),
            speed: f32::from_le_bytes(buf[244..248].try_into().unwrap()),
            best_lap: f32::from_le_bytes(buf[284..288].try_into().unwrap()),
            current_lap: f32::from_le_bytes(buf[292..296].try_into().unwrap()),
            current_race_time: f32::from_le_bytes(buf[296..300].try_into().unwrap()),
            gear: buf[307] as u8
        };
    }
}