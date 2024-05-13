use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use local_ip_address::local_ip;


pub struct PacketInfo {
    current_rpm: f32,
    speed: f32,
    best_lap: f32,
    current_lap: f32,
    current_race_time: f32,
    gear: u8
}

impl PacketInfo {
    pub fn get_current_rpm(&self) -> f32 {
        return self.current_rpm
    }

    pub fn get_speed(&self) -> f32 {
        return self.speed
    }

    pub fn get_best_lap(&self) -> f32 {
        return self.best_lap
    }

    pub fn get_current_lap(&self) -> f32 {
        return self.current_lap
    }

    pub fn get_current_race_time(&self) -> f32 {
        return self.current_race_time
    }

    pub fn get_gear(&self) -> i32 {
        return self.gear as i32
    }
}

fn setup_udp_socket() -> UdpSocket {
    let ip_addr: String = local_ip().unwrap().to_string();
    let port: &str = "8080";
    let binding_addr: String = format!("{}:{}", ip_addr, port);
    let socket: UdpSocket = UdpSocket::bind(binding_addr).expect("Failed to bind to address");
    println!("Successfully binded to udp socket");
    return socket
}

pub fn parse_packets(sender: Sender<PacketInfo>) -> PacketInfo {
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

        sender.send(packet_info).expect("Error sending packet data to thread");
    }
}