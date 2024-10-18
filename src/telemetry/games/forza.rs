use std::net::UdpSocket;
use std::sync::{Arc, Condvar, Mutex};

use crate::telemetry::parser::TelemetryParser;
use crate::telemetry::utils::{parse_f32_from_bytes, parse_i16_from_bytes, setup_udp_socket};

#[derive(Default, Clone)]
pub struct ForzaTelemetry {
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

impl ForzaTelemetry {
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
}

pub struct ForzaParser;

impl TelemetryParser for ForzaParser {
    type GameTelemetry = ForzaTelemetry;
    fn parse_packets(telemetry: Arc<(Mutex<Self::GameTelemetry>, Condvar)>) {
        let socket: UdpSocket = setup_udp_socket();

        loop {
            let mut buf: Vec<u8> = vec![0; 500];

            socket.recv_from(&mut buf).unwrap();
            let (lock, cvar) = &*telemetry;
            let mut telem = lock.lock().unwrap();

            telem.current_rpm = parse_f32_from_bytes(&buf[16..20]).round();
            telem.max_rpm = parse_f32_from_bytes(&buf[8..12]);
            telem.speed = (parse_f32_from_bytes(&buf[244..248]) * 2.237).round();
            telem.best_lap = parse_f32_from_bytes(&buf[284..288]);
            telem.current_lap = parse_f32_from_bytes(&buf[292..296]);
            telem.last_lap = parse_f32_from_bytes(&buf[288..292]);
            telem.lap_number = parse_i16_from_bytes(&buf[300..302]) as i32;
            telem.position = buf[302] as i32;
            telem.gear = buf[307] as i32;
            telem.accel = buf[303] as f32;
            telem.brake = buf[304] as f32;
            telem.temp_left_f = parse_f32_from_bytes(&buf[256..260]).round();
            telem.temp_right_f = parse_f32_from_bytes(&buf[260..264]).round();
            telem.temp_left_r = parse_f32_from_bytes(&buf[264..268]).round();
            telem.temp_right_r = parse_f32_from_bytes(&buf[268..272]).round();

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
