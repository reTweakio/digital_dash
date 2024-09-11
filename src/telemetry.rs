use local_ip_address::local_ip;
use std::net::UdpSocket;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Clone, Default)]
pub struct Telemetry {
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
    lap_number: i32,
}

impl Telemetry {
    pub fn get_current_rpm(&self) -> f32 {
        self.current_rpm
    }

    pub fn get_max_rpm(&self) -> f32 {
        self.max_rpm
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_best_lap(&self) -> String {
        Self::format_time(self.best_lap)
    }

    pub fn get_current_lap(&self) -> String {
        Self::format_time(self.current_lap)
    }

    pub fn get_gear(&self) -> i32 {
        self.gear
    }

    pub fn get_accel(&self) -> f32 {
        self.accel / 255.0 * 100.0
    }

    pub fn get_brake(&self) -> f32 {
        self.brake / 255.0 * 100.0
    }

    pub fn get_position(&self) -> i32 {
        self.position
    }

    pub fn get_temp_left_f(&self) -> f32 {
        self.temp_left_f
    }

    pub fn get_temp_right_f(&self) -> f32 {
        self.temp_right_f
    }

    pub fn get_temp_left_r(&self) -> f32 {
        self.temp_left_r
    }

    pub fn get_temp_right_r(&self) -> f32 {
        self.temp_right_r
    }

    pub fn get_lap_number(&self) -> i32 {
        self.lap_number + 1
    }

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
        } else {
            format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
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

    pub fn parse_packets(telem: Arc<(Mutex<Telemetry>, Condvar)>) {
        let socket: UdpSocket = Self::setup_udp_socket();

        loop {
            let mut buf: Vec<u8> = vec![0; 500];

            match socket.recv_from(&mut buf) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                }
            }
            let (lock, cvar) = &*telem;
            let mut telem = lock.lock().unwrap();

            telem.current_rpm = Self::parse_f32_from_bytes(&buf[16..20]).round();
            telem.max_rpm = Self::parse_f32_from_bytes(&buf[8..12]);
            telem.speed = (Self::parse_f32_from_bytes(&buf[244..248]) * 2.237).round();
            telem.best_lap = Self::parse_f32_from_bytes(&buf[284..288]);
            telem.current_lap = Self::parse_f32_from_bytes(&buf[292..296]);
            telem.last_lap = Self::parse_f32_from_bytes(&buf[288..292]);
            telem.lap_number = Self::parse_i16_from_bytes(&buf[300..302]) as i32;
            telem.position = buf[302] as i32;
            telem.gear = buf[307] as i32;
            telem.accel = buf[303] as f32;
            telem.brake = buf[304] as f32;
            telem.temp_left_f = Self::parse_f32_from_bytes(&buf[256..260]).round();
            telem.temp_right_f = Self::parse_f32_from_bytes(&buf[260..264]).round();
            telem.temp_left_r = Self::parse_f32_from_bytes(&buf[264..268]).round();
            telem.temp_right_r = Self::parse_f32_from_bytes(&buf[268..272]).round();

            if telem.prev_best == 0.0 && telem.lap_number > 1 {
                telem.prev_best = telem.best_lap;
            }

            if telem.last_lap != telem.best_lap {
                telem.prev_best = telem.best_lap;
            }

            cvar.notify_one();
            let _telem = cvar.wait(telem).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_calculation() {
        let mut telemetry = Telemetry::default();

        // Delta when last lap is worse (slower) and no change in best lap
        assert_eq!(telemetry.get_delta(), "00:02.000");

        // Update values to reflect a faster lap time
        telemetry.best_lap = 60.0;
        telemetry.last_lap = 58.0;
        telemetry.prev_best = 62.0;

        // Delta when last lap is faster (better) than best lap
        assert_eq!(telemetry.get_delta(), "-00:02.000");

        // Update values for the case where last lap time is the same as the best lap time
        telemetry.best_lap = 60.0;
        telemetry.last_lap = 62.0;
        telemetry.prev_best = 62.0;

        // Delta when last lap is worse (slower) than best lap but prev_best is the same as best_lap
        assert_eq!(telemetry.get_delta(), "00:02.000");

        // Update values for the case where last lap time is equal to the best lap time
        telemetry.best_lap = 60.0;
        telemetry.last_lap = 60.0;
        telemetry.prev_best = 62.0;

        // Delta when last lap is the same as best lap and prev_best is different
        assert_eq!(telemetry.get_delta(), "-00:02.000");
    }
}
