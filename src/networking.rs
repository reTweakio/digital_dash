use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use local_ip_address::local_ip;


pub struct PacketInfo {
    current_rpm: f32,
    max_rpm: f32,
    speed: f32,
    best_lap: f32,
    prev_best: f32,
    current_lap: f32,
    last_lap: f32,
    gear: i32,
    accel: f32,
    brake: f32,
    position: i32,
    temp_left_f: f32,
    temp_right_f: f32,
    temp_left_r: f32,
    temp_right_r: f32,
    lap_number: i32
}

impl PacketInfo {
    pub fn get_current_rpm(&self) -> f32 { self.current_rpm }

    pub fn get_max_rpm(&self) -> f32 { self.max_rpm }

    pub fn get_speed(&self) -> f32 { self.speed }

    pub fn get_best_lap(&self) -> String { Self::format_time(self.best_lap) }

    pub fn get_current_lap(&self) -> String { Self::format_time(self.current_lap) }

    pub fn get_gear(&self) -> i32 { self.gear }

    pub fn get_accel(&self) -> f32 { self.accel / 255.0 * 100.0 }

    pub fn get_brake(&self) -> f32 { self.brake / 255.0 * 100.0 }

    pub fn get_position(&self) -> i32 { self.position }

    pub fn get_temp_left_f(&self) -> f32 { self.temp_left_f }

    pub fn get_temp_right_f(&self) -> f32 { self.temp_right_f }

    pub fn get_temp_left_r(&self) -> f32 { self.temp_left_r }

    pub fn get_temp_right_r(&self) -> f32 { self.temp_right_r }

    pub fn get_lap_number(&self) -> i32 { self.lap_number + 1 }

    pub fn get_delta(&self) -> String { 
        let delta = if self.last_lap == self.best_lap {
            self.last_lap - self.prev_best
        } else {
            self.last_lap - self.best_lap
        };

        Self::format_time(delta)
    }

    fn format_time(time: f32) -> String {
        let minutes: i32 = (time.abs() / 60.0).floor() as i32;
        let seconds: i32 = (time.abs() % 60.0).floor() as i32;
        let milliseconds: i32 = (time.abs() * 1000.0).round() as i32 % 1000;

        if time < 0.0 {
            format!("-{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
        } 

        else {
            format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
        }
    }
}


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

pub fn parse_packets(sender: Sender<PacketInfo>) {
    let socket: UdpSocket = setup_udp_socket();
    let mut buf: Vec<u8> = vec![0; 500];
    let mut prev_best: f32 = f32::MAX;

    loop {
        match socket.recv_from(&mut buf) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }

        let packet_info: PacketInfo = PacketInfo {
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