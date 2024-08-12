use local_ip_address::local_ip;
use std::net::UdpSocket;
use std::sync::mpsc::Sender;

use crate::telemetry::TelemPacket;

fn setup_udp_socket() -> UdpSocket {
    let ip_addr: String = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };
    let port: &str = "8080";
    let binding_addr: String = format!("{}:{}", ip_addr, port);
    let socket: UdpSocket = match UdpSocket::bind(binding_addr) {
        Ok(socket) => socket,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };
    socket
}

fn parse_f32_from_bytes(buf: &[u8]) -> f32 {
    f32::from_le_bytes(match buf.try_into() {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    })
}

fn parse_i16_from_bytes(buf: &[u8]) -> i16 {
    i16::from_le_bytes(match buf.try_into() {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    })
}

pub fn parse_packets(sender: Sender<TelemPacket>) {
    let socket: UdpSocket = setup_udp_socket();
    let mut buf: Vec<u8> = vec![0; 500];
    let mut prev_best: f32 = 0.0;

    loop {
        match socket.recv_from(&mut buf) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }

        let packet_info: TelemPacket = TelemPacket {
            current_rpm: parse_f32_from_bytes(&buf[16..20]).round(),
            max_rpm: parse_f32_from_bytes(&buf[8..12]),
            speed: (parse_f32_from_bytes(&buf[244..248]) * 2.237).round(),
            best_lap: parse_f32_from_bytes(&buf[284..288]),
            prev_best: prev_best,
            current_lap: parse_f32_from_bytes(&buf[292..296]),
            last_lap: parse_f32_from_bytes(&buf[288..292]),
            lap_number: parse_i16_from_bytes(&buf[300..302]) as i32,
            position: buf[302] as i32,
            gear: buf[307] as i32,
            accel: buf[303] as f32,
            brake: buf[304] as f32,
            temp_left_f: parse_f32_from_bytes(&buf[256..260]).round(),
            temp_right_f: parse_f32_from_bytes(&buf[260..264]).round(),
            temp_left_r: parse_f32_from_bytes(&buf[264..268]).round(),
            temp_right_r: parse_f32_from_bytes(&buf[268..272]).round(),
        };

        if packet_info.lap_number == 2 {
            prev_best = packet_info.best_lap;
        }

        if packet_info.last_lap != packet_info.best_lap {
            prev_best = packet_info.best_lap;
        }

        match sender.send(packet_info) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }
}
